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
    pub crc_name: String,
}

pub trait Chunk {
    fn get_serial(&self) -> Result<String>;
    fn get_size(&self) -> Result<u64>;
    fn count(&self) -> Result<u8>;
}

impl Chunk for ISOChunk {
    fn get_serial(&self) -> Result<String> {
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

    fn get_size(&self) -> Result<u64> {
        let metadata = fs::metadata(&self.path)?;
        Ok(metadata.len())
    }

    fn count(&self) -> Result<u8> {
        Ok(0)
    }
}

impl Chunk for GameChunk {
    fn get_serial(&self) -> Result<String> {
        let chunks = list_game_chunks(&self.path, &self.crc_name)?;
        let serial = chunks.get(0).unwrap().split('.').collect::<Vec<&str>>();
        Ok(format!("{}.{}", serial[2], serial[3]))
    }

    fn get_size(&self) -> Result<u64> {
        let chunks = list_game_chunks(&self.path, &self.crc_name)?;
        let total_size = chunks
            .into_iter()
            .map(|e| fs::metadata(Path::new(&self.path).join(e)).unwrap().len())
            .collect::<Vec<u64>>()
            .iter()
            .sum();

        Ok(total_size)
    }

    fn count(&self) -> Result<u8> {
        let chunks = list_game_chunks(&self.path, &self.crc_name)?;
        Ok(chunks.len() as u8)
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
