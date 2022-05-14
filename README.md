# ulmake

A command line tool that helps to create and manage PlayStation 2 games in USBAdvance/Extreme format, similarly to `USB Util`. Current features include:

* Adding new games to USB with `add`. This ensures that Dual Layer DVD9 images are split to satisfy FAT32 file size limitations, and game entry is correctly written to `ul.cfg` file.
* Deleting games from USB with `delete`. This removes all DVD9 image chunks related to particular game from USB, and deletes game entry from `ul.cfg`.
* Listing current games on USB with `list`.

## Build from source

```
git clone https://github.com/xadrianzetx/ulmake.git && cd ulmake
cargo build --release
```
## Usage

```
USAGE:
    ulmake [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add       Creates USBAdvance/Extreme format PlayStation 2 game
              from .iso and registers it in ul.cfg file
    delete    Removes PlayStation 2 game from ul.cfg along with ul. chunks
              Game can be removed either by ul.cfg index or by OPL name
    help      Prints this message or the help of the given subcommand(s)
    list      Lists current entries in ul.cfg
```

Run `ulmake [SUBCOMMAND] --help` to see the arguments of a specific subcommand.