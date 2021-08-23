mod game;
mod option;
mod ul;

use clap::{App, Arg};
use std::env::current_dir;
use std::path::{Path, PathBuf};

fn main() {
    let matches = App::new("ulmake")
        .version("0.1.1")
        .author("xadrianzetx")
        .about(concat!(
            "A CLI utility that helps to manage PlayStation 2 games\n",
            "in USBAdvance/Extreme format (similarly to USB Util)"
        ))
        .subcommand(
            App::new("add")
                .about(concat!(
                    "Creates USBAdvance/Extreme format PlayStation 2 game\n",
                    "from .iso and registers it in ul.cfg file"
                ))
                .args(&[
                    Arg::with_name("image")
                        .index(1)
                        .takes_value(true)
                        .required(true)
                        .help("Path to PlayStation 2 .iso disk image"),
                    Arg::with_name("ulpath")
                        .index(2)
                        .takes_value(true)
                        .required(true)
                        .help(concat!(
                            "Directory where game should be created\n",
                            "If ul.cfg is not found there, new one will be created"
                        )),
                    Arg::with_name("name")
                        .short("n")
                        .takes_value(true)
                        .help(concat!(
                            "Name under which game will be visible in OPL\n",
                            "Must be <= 32 characters\n",
                            "If not specified, .iso name is taken"
                        )),
                ]),
        )
        .subcommand(
            App::new("delete")
                .about(concat!(
                    "Removes PlayStation 2 game from ul.cfg along with ul. chunks\n",
                    "Game can be removed either by ul.cfg index or by OPL name"
                ))
                .args(&[
                    Arg::with_name("ulpath")
                        .index(1)
                        .takes_value(true)
                        .required(true)
                        .help(concat!(
                            "Directory from which game should be deleted\n",
                            "Must contain valid ul.cfg file"
                        )),
                    Arg::with_name("index")
                        .required_unless("name")
                        .conflicts_with("name")
                        .short("i")
                        .takes_value(true)
                        .help(concat!(
                            "ul.cfg index of game that is to be deleted\n",
                            "Use `ulmake list` to get valid indices"
                        )),
                    Arg::with_name("name")
                        .required_unless("index")
                        .conflicts_with("index")
                        .short("n")
                        .takes_value(true)
                        .help(concat!(
                            "OPL name of game that is to be deleted\n",
                            "Use `ulmake list` to get valid names"
                        )),
                ]),
        )
        .subcommand(
            App::new("list")
                .about("Lists current entries in ul.cfg")
                .arg(
                    Arg::with_name("ulpath")
                        .index(1)
                        .takes_value(true)
                        .help(concat!(
                            "Directory containing ul.cfg file\n",
                            "Defaults to current dir if not specified"
                        )),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        let isopath = Path::new(matches.value_of("image").unwrap());
        let dstpath = Path::new(matches.value_of("ulpath").unwrap());
        let opl_name = match matches.value_of("name") {
            Some(n) => String::from(n),
            None => {
                let isoname = isopath.file_stem().unwrap();
                String::from(isoname.to_str().unwrap())
            }
        };

        match option::add_game(&isopath, &dstpath, opl_name) {
            Ok(()) => (),
            Err(_) => println!("Could not create the game"),
        }
    }

    if let Some(matches) = matches.subcommand_matches("delete") {
        let path = Path::new(matches.value_of("ulpath").unwrap());
        match matches.value_of("index") {
            Some(index) => {
                let uidx = index.parse::<usize>().unwrap();
                match option::delete_game_by_index(path, uidx) {
                    Ok(()) => (),
                    Err(_) => println!("Could not delete game by index"),
                }
            }
            None => (),
        }

        match matches.value_of("name") {
            Some(name) => {
                let namestr = String::from(name);
                match option::delete_game_by_name(path, namestr) {
                    Ok(()) => (),
                    Err(_) => println!("Could not delete game by name"),
                }
            }
            None => (),
        }
    }

    if let Some(matches) = matches.subcommand_matches("list") {
        let pbuff = match matches.value_of("ulpath") {
            Some(p) => PathBuf::from(p),
            None => current_dir().unwrap(),
        };

        match option::list_games(&pbuff.as_path()) {
            Ok(()) => (),
            Err(_) => println!("Could not load ul.cfg"),
        }
    }
}
