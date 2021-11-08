use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

use iso9660::{DirectoryEntry, ISO9660};
use regex::Regex;

const SYSTEM_CNF_PATH: &str = "/SYSTEM.CNF";

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