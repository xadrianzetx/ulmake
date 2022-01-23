use crate::ul::Ulcfg;

use fs2;
use std::env::current_dir;
use std::fs::canonicalize;
use std::io::Result;
use std::path::{Path, PathBuf};

use clap::ArgMatches;

fn list_games(path: &Path) -> Result<()> {
    let ulpath = path.join(Path::new("ul.cfg"));
    let ulcfg = Ulcfg::load(&ulpath)?;

    let free = fs2::available_space(&path)? as f64;
    let realpath = canonicalize(&path)?;

    println!("ul.cfg at {}", realpath.display());
    println!("Available space: {:.2}GB", free / 1000000000.0);
    ulcfg.list_games();

    Ok(())
}

pub fn list(args: &ArgMatches) {
    let path = match args.value_of("ulpath") {
        Some(p) => PathBuf::from(p),
        None => current_dir().unwrap(),
    };

    match list_games(path.as_path()) {
        Ok(()) => (),
        Err(_) => println!("Could not load ul.cfg"),
    };
}
