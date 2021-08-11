use crate::ul::Ulcfg;

use fs2;
use std::fs::metadata;
use std::path::Path;
use std::io::{Result, Error, ErrorKind};

pub fn list_games(path: &Path) -> Result<()> {
    let ulpath = path.join(Path::new("ul.cfg"));
    let ulcfg = Ulcfg::load(&ulpath)?;
    ulcfg.list_games();

    Ok(())
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


    println!("Creating {} from {:?}", name, isopath.file_name().unwrap());
    ulcfg.add_game(&isopath, &dstpath, name)?;
    ulcfg.save(&ulpath)?;

    Ok(())
}

pub fn delete_game_by_name(path: &Path, name: &String) -> Result<()> {
    let ulpath = path.join(Path::new("ul.cfg"));
    let mut ulcfg = Ulcfg::load(&ulpath)?;
    ulcfg.delete_game_by_name(name, path)?;
    ulcfg.save(&ulpath)?;

    Ok(())
}

pub fn delete_game_by_index(path: &Path, index: usize) -> Result<()> {
    let ulpath = path.join(Path::new("ul.cfg"));
    let mut ulcfg = Ulcfg::load(&ulpath)?;
    ulcfg.delete_game_by_index(index, path)?;
    ulcfg.save(&ulpath)?;

    Ok(())
}