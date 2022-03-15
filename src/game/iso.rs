use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

use iso9660::{DirectoryEntry, ISO9660};
use regex::Regex;

const SYSTEM_CNF_PATH: &str = "/SYSTEM.CNF";

pub struct ISOChunk {
    pub path: PathBuf,
}

pub struct GameChunk {
    pub path: PathBuf,
}

pub trait Chunk {
    fn serial(&self) -> Result<String>;
    fn size(&self) -> Result<u64>;
}

impl From<PathBuf> for ISOChunk {
    fn from(path: PathBuf) -> Self {
        ISOChunk { path }
    }
}

impl From<PathBuf> for GameChunk {
    fn from(path: PathBuf) -> Self {
        GameChunk { path }
    }
}

impl Chunk for ISOChunk {
    fn serial(&self) -> Result<String> {
        let isofile = File::open(&self.path)?;
        let iso = match ISO9660::new(isofile) {
            Ok(image) => image,
            Err(_) => return Err(Error::from(ErrorKind::InvalidData)),
        };

        match iso.open(SYSTEM_CNF_PATH).unwrap() {
            Some(DirectoryEntry::File(file)) => {
                let mut buffer = String::new();
                file.read().read_to_string(&mut buffer)?;

                // first line of SYSTEM.CNF should go like:
                // BOOT2 = cdrom0:\SLXS_XXX.XX;1
                // and we are fetching SLXS_XXX.XX from it
                let re = Regex::new("([^:\\\\;]+)(:?;1)?$").unwrap();
                let boot_path = buffer.lines().next().unwrap();
                let boot_file = re.captures(boot_path).unwrap();

                let serial = boot_file.get(1).unwrap().as_str();
                Ok(String::from(serial))
            }
            _ => Err(Error::from(ErrorKind::NotFound)),
        }
    }

    fn size(&self) -> Result<u64> {
        let metadata = fs::metadata(&self.path)?;
        Ok(metadata.len())
    }
}

impl Chunk for GameChunk {
    fn serial(&self) -> Result<String> {
        fs::metadata(&self.path)?;
        let segments = self
            .path
            .file_name()
            .and_then(|c| c.to_str())
            .ok_or(ErrorKind::InvalidData)?
            .split('.')
            .collect::<Vec<&str>>();

        if segments.len() != 5 {
            // Chunk names have five comma separated segments (including extension).
            // Example: ul.84BA9D95.SLXS_123.45.00
            return Err(Error::from(ErrorKind::InvalidData));
        }

        Ok(format!("{}.{}", segments[2], segments[3]))
    }

    fn size(&self) -> Result<u64> {
        let metadata = fs::metadata(&self.path)?;
        Ok(metadata.len())
    }
}

fn list_game_chunks(path: &Path, crc_name: &str) -> Result<Vec<String>> {
    let chunks = fs::read_dir(path)?
        .map(|res| res.unwrap().file_name().into_string().unwrap())
        .filter(|n| n.contains(crc_name))
        .collect::<Vec<_>>();

    if chunks.is_empty() {
        return Err(Error::from(ErrorKind::NotFound));
    }

    Ok(chunks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_isochunk_get_serial() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/testimage.iso");
        let isochunk = ISOChunk { path };
        let serial = isochunk.serial().unwrap();
        assert_eq!(serial, String::from("SLXS_123.45"));
    }

    #[test]
    fn test_isochunk_get_serial_file_not_found() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/foo.iso");
        let isochunk = ISOChunk { path };
        let serial = isochunk.serial();
        assert!(serial.is_err());
    }

    #[test]
    fn test_isochunk_get_size() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/testimage.iso");
        let isochunk = ISOChunk { path };
        let size = isochunk.size().unwrap();
        let expected_size = 358400;
        assert_eq!(size, expected_size);
    }

    #[test]
    fn test_isochunk_get_size_file_not_found() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/foo.iso");
        let isochunk = ISOChunk { path };
        let size = isochunk.size();
        assert!(size.is_err());
    }

    #[test]
    fn test_gamechunk_get_serial() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        let crc_name = String::from("84BA9D95");
        let gamechunk = GameChunk { path, crc_name };
        let serial = gamechunk.serial().unwrap();
        assert_eq!(serial, String::from("SLXS_123.45"))
    }

    #[test]
    fn test_gamechunk_get_serial_file_not_found() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        let crc_name = String::from("00000000");
        let gamechunk = GameChunk { path, crc_name };
        let serial = gamechunk.serial();
        assert!(serial.is_err());
    }

    #[test]
    fn test_gamechunk_get_size() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        let crc_name = String::from("84BA9D95");
        let gamechunk = GameChunk { path, crc_name };
        let size = gamechunk.size().unwrap();
        let expected_size = 20;
        assert_eq!(size, expected_size);
    }

    #[test]
    fn test_gamechunk_get_size_file_not_found() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        let crc_name = String::from("00000000");
        let gamechunk = GameChunk { path, crc_name };
        let size = gamechunk.size();
        assert!(size.is_err());
    }

    #[test]
    fn test_list_game_chunks() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        let chunks = list_game_chunks(&path, "84BA9D95").unwrap();
        assert_eq!(chunks.len(), 2);
    }

    #[test]
    fn test_list_game_chunks_file_not_found() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        let chunks = list_game_chunks(&path, "00000000");
        assert!(chunks.is_err());
    }
}
