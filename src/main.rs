mod commands;
mod game;
mod ul;

use clap::{App, Arg};

fn main() {
    let matches = App::new("ulmake")
        .version("0.3.0")
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

    match matches.subcommand() {
        ("add", Some(args)) => commands::add::add(args),
        ("delete", Some(args)) => commands::delete::delete(args),
        ("list", Some(args)) => commands::list::list(args),
        _ => (),
    }
}
