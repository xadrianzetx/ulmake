# ulmake

`ulmake` is a CLI tool helps to create and manage PlayStation2 games in USBAdvance/Extreme format, similarly to `USB Util`. Current features:

* adding new games to USB with `add`. This ensures that Dual Layer DVD9 images are split to satisfy FAT32 file size limitations, and game entry is correctly added to `ul.cfg` file.
* deleting games from USB with `delete`. This removes all DVD9 image chunks related to particular game from USB, and deletes game entry from `ul.cfg`.
* listing current games on USB with `list`.

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