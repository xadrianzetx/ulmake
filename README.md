# ulmake

This CLI tool helps to create and manage PlayStation2 games in USB Advance/Extreme format, similarly to `USB Util`. Current features:

* adding new games to USB with `add`. This ensures that Dual Layer DVD9 images are split to satisfy FAT32 file size limitations, and game entry is correctly added to `ul.cfg` file.
* deleting games from USB with `delete`. This removes all DVD9 image chunks related to particular game from USB, and deletes game entry from `ul.cfg`.
* listing current games on USB with `list`.

## Usage

```
ulmake 0.1.0
xadrianzetx
A CLI utility that helps to manage PS2 games in .ul format (similarly to USB Util)

USAGE:
    ulmake [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add       Adds PS2 game to ul.cfg along with FAT32 compliant game image chunks
    delete    Removes PS2 game from ul.cfg along with image chunks
    help      Prints this message or the help of the given subcommand(s)
    list      Lists current entries in ul.cfg
```