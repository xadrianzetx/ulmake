use crate::ul::Ulcfg;

use fs2;
use std::fs::metadata;
use std::path::Path;
use std::io::{Result, Error, ErrorKind};

pub fn list_games() {
    unimplemented!();
}

pub fn add_game(isopath: &Path, dstpath: &Path, name: String) -> Result<()> {
    let ulpath = dstpath.join(Path::new("ul.cfg"));
    let dstspace = fs2::available_space(&dstpath)?;
    let iso = metadata(&isopath)?;

    if iso.len() >= dstspace {
        return Err(Error::from(ErrorKind::OutOfMemory))
    }

    let mut ulcfg = match ulpath.exists() {
        true => Ulcfg::load(&ulpath)?,
        false => Ulcfg::new().unwrap()
    };


    println!("Creating {} from {:?}", name, isopath.file_name());
    ulcfg.add_game(&isopath, &dstpath, name)?;
    ulcfg.save(&ulpath)?;

    Ok(())
}

pub fn delete_game() {
    unimplemented!();
}