mod ul;
mod game;
mod option;

use std::path::Path;
use clap::{App, Arg};

fn main() {
    let matches = App::new("ulmake")
        .version("0.1.0")
        .author("xadrianzetx")
        .about("TODO")
        .subcommand(
            App::new("add")
            .args(&[
                Arg::with_name("image")
                    .short("i")
                    .takes_value(true)
                    .required(true),
                Arg::with_name("name")
                    .short("n")
                    .takes_value(true)
                    .required(true),
                Arg::with_name("ulpath")
                    .short("p")
                    .takes_value(true)
                    .required(true)
            ])
        )
        .subcommand(
            App::new("delete")
            .args(&[
                Arg::with_name("index")
                    .required_unless("name")
                    .conflicts_with("name")
                    .short("i")
                    .takes_value(true),
                Arg::with_name("name")
                    .required_unless("index")
                    .conflicts_with("index")
                    .short("n")
                    .takes_value(true),
                Arg::with_name("ulpath")
                    .short("p")
                    .takes_value(true)
                    .required(true)
            ])
        )
        .subcommand(
            App::new("list")
            .arg(
                Arg::with_name("ulpath")
                    .short("p")
                    .takes_value(true)
                    .required(true)
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