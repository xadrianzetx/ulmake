use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use regex::Regex;
use iso9660::{ISO9660, DirectoryEntry};

const SYSTEM_CNF_PATH: &str = "/SYSTEM.CNF";


pub fn get_serial_from_iso(path: &Path) -> Option<String> {
    unimplemented!();
}