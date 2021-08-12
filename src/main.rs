mod ul;
mod game;
mod option;

use std::path::Path;
use clap::{App, Arg};

fn main() {
    let matches = App::new("ulmake")
        .version("0.1.0")
        .author("xadrianzetx")
        .about("A CLI utility that helps to manage PS2 games in .ul format (similarly to USB Util)")
        .subcommand(
            App::new("add")
            .about("Adds PS2 game to ul.cfg along with FAT32 compliant game image chunks")
            .args(&[
                Arg::with_name("image")
                    .short("i")
                    .takes_value(true)
                    .required(true)
                    .help("Path to PS2 .iso disk image"),
                Arg::with_name("name")
                    .short("n")
                    .takes_value(true)
                    .required(true)
                    .help("Name under which game will be visible in OPL. Must be <= 32 characters"),
                Arg::with_name("ulpath")
                    .short("p")
                    .takes_value(true)
                    .required(true)
                    .help("Path to directory with ul.cfg file. If there is no such file there, new one will be created")
            ])
        )
        .subcommand(
            App::new("delete")
            .about("Removes PS2 game from ul.cfg along with image chunks")
            .args(&[
                Arg::with_name("index")
                    .required_unless("name")
                    .conflicts_with("name")
                    .short("i")
                    .takes_value(true)
                    .help("ul.cfg index of game that is to be deleted. Use `ulmake list` to get valid indices"),
                Arg::with_name("name")
                    .required_unless("index")
                    .conflicts_with("index")
                    .short("n")
                    .takes_value(true)
                    .help("OPL name of game that is to be deleted. Use `ulmake list` to get valid names"),
                Arg::with_name("ulpath")
                    .short("p")
                    .takes_value(true)
                    .required(true)
                    .help("Path to directory with ul.cfg file")
            ])
        )
        .subcommand(
            App::new("list")
            .about("Lists current entries in ul.cfg")
            .arg(
                Arg::with_name("ulpath")
                    .short("p")
                    .takes_value(true)
                    .required(true)
                    .help("Path to directory with ul.cfg file")
            )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        let isopath = Path::new(matches.value_of("image").unwrap());
        let dstpath = Path::new(matches.value_of("ulpath").unwrap());
        let opl_name = String::from(matches.value_of("name").unwrap());

        match option::add_game(&isopath, &dstpath, opl_name) {
            Ok(()) => (),
            Err(_) => println!("Could not create the game")
        }
    }

    if let Some(matches) = matches.subcommand_matches("delete") {
        let path = Path::new(matches.value_of("ulpath").unwrap());
        match matches.value_of("index") {
            Some(index) => {
                let uidx = index.parse::<usize>().unwrap();
                match option::delete_game_by_index(path, uidx) {
                    Ok(()) => (),
                    Err(_) => println!("Could not delete game by index")
                }
            },
            None => ()
        }

        match matches.value_of("name") {
            Some(name) => {
                let namestr = String::from(name);
                match option::delete_game_by_name(path, namestr) {
                    Ok(()) => (),
                    Err(_) => println!("Could not delete game by name")
                }
            },
            None => ()
        }
    }

    if let Some(matches) = matches.subcommand_matches("list") {
        let path = Path::new(matches.value_of("ulpath").unwrap());
        match option::list_games(&path) {
            Ok(()) => (),
            Err(_) => println!("Could not load ul.cfg")
        }
    }
}