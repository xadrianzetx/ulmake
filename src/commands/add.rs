use crate::ul::Ulcfg;

use std::fs::metadata;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

use clap::ArgMatches;

fn add_game(isopath: &Path, dstpath: &Path, name: String) -> Result<()> {
    let ulpath = dstpath.join(Path::new("ul.cfg"));
    let dstspace = fs2::available_space(&dstpath)?;
    let iso = metadata(&isopath)?;

    if iso.len() >= dstspace {
        return Err(Error::from(ErrorKind::OutOfMemory));
    }

    if name.len() > 32 {
        // OPL name cannot be longer than 32 bytes
        return Err(Error::from(ErrorKind::InvalidInput));
    }

    let mut ulcfg = match ulpath.exists() {
        true => Ulcfg::load(&ulpath)?,
        false => Ulcfg::new(),
    };

    println!("Creating {} from {:?}", name, isopath.file_name().unwrap());
    ulcfg.add_game(isopath, dstpath, name)?;
    ulcfg.save(&ulpath)?;

    Ok(())
}

pub fn add(args: &ArgMatches) {
    let isopath = Path::new(args.value_of("image").unwrap());
    let dstpath = Path::new(args.value_of("ulpath").unwrap());
    let opl_name = match args.value_of("name") {
        Some(n) => String::from(n),
        None => {
            let isoname = isopath.file_stem().unwrap();
            String::from(isoname.to_str().unwrap())
        }
    };

    match add_game(isopath, dstpath, opl_name) {
        Ok(()) => (),
        Err(_) => println!("Could not create the game"),
    }
}
