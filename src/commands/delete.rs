use crate::ul::Ulcfg;

use std::io::Result;
use std::path::Path;

use clap::ArgMatches;

fn delete_game_by_name(path: &Path, name: &str) -> Result<()> {
    let ulpath = path.join(Path::new("ul.cfg"));
    let mut ulcfg = Ulcfg::load(&ulpath)?;
    ulcfg.delete_game_by_name(name)?;
    ulcfg.save(&ulpath)?;

    Ok(())
}

fn delete_game_by_index(path: &Path, index: usize) -> Result<()> {
    let ulpath = path.join(Path::new("ul.cfg"));
    let mut ulcfg = Ulcfg::load(&ulpath)?;
    ulcfg.delete_game_by_index(index)?;
    ulcfg.save(&ulpath)?;

    Ok(())
}

pub fn delete(args: &ArgMatches) {
    let path = Path::new(args.value_of("ulpath").unwrap());
    if let Some(index) = args.value_of("index") {
        let uidx = index.parse::<usize>().unwrap();
        match delete_game_by_index(path, uidx) {
            Ok(()) => (),
            Err(_) => println!("Could not delete game by index"),
        }
    }

    if let Some(name) = args.value_of("name") {
        match delete_game_by_name(path, name) {
            Ok(()) => (),
            Err(_) => println!("Could not delete game by name"),
        }
    }
}
