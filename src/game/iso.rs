use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

use iso9660::{DirectoryEntry, ISO9660};
use regex::Regex;

const SYSTEM_CNF_PATH: &str = "/SYSTEM.CNF";

fn list_game_chunks(path: &Path, crc_name: &str) -> Result<Vec<String>> {
    let chunks = fs::read_dir(path)?
        .map(|res| res.unwrap().file_name().into_string().unwrap())
        .filter(|n| n.contains(crc_name))
        .collect::<Vec<_>>();

    if chunks.len() == 0 {
        return Err(Error::from(ErrorKind::NotFound));
    }

    Ok(chunks)
}

pub fn get_serial_from_iso(path: &Path) -> Result<String> {
    let isofile = File::open(path)?;
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
            let boot_path = buffer.lines().nth(0).unwrap();
            let boot_file = re.captures(boot_path).unwrap();

            let serial = boot_file.get(1).unwrap().as_str();
            Ok(String::from(serial))
        }
        _ => Err(Error::from(ErrorKind::NotFound)),
    }
}

pub fn get_size_from_iso(isopath: &Path) -> Result<u64> {
    let metadata = fs::metadata(isopath)?;
    Ok(metadata.len())
}

pub fn get_size_from_chunks(dir: &Path, crc_name: &str) -> Result<u64> {
    let mut total_size = 0;
    let chunk_names = list_game_chunks(dir, crc_name)?;

    for chunk_name in chunk_names.iter() {
        let filepath = Path::new(dir).join(chunk_name);
        let metadata = fs::metadata(filepath)?;
        total_size += metadata.len();
    }

    Ok(total_size)
}
