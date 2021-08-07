mod crc;
mod serial;

use std::path::Path;
use std::io::Result;

const GAME_PREFIX: &str = "ul.";

pub struct Game {
    crc_name: String,
    pub opl_name: String,
    pub serial: String,
    pub num_chunks: Option<i32>
}

impl Game {
    pub fn from_iso(isopath: &Path, opl_name: String) -> Result<Game> {
        let crc_name = crc::get_game_name_crc(&opl_name);
        let serial = match serial::get_serial_from_iso(isopath) {
            Ok(s) => s,
            Err(e) => return Err(e) 
        };

        let game = Game {
            opl_name: opl_name,
            crc_name: crc_name,
            serial: serial,
            num_chunks: None
        };

        Ok(game)
    }

    pub fn from_config(opl_name: String, serial: String, num_chunks: i32) -> Result<Game> {
        let crc_name = crc::get_game_name_crc(&opl_name);
        let game = Game {
            opl_name: opl_name,
            crc_name: crc_name,
            serial: serial,
            num_chunks: Some(num_chunks)
        };

        Ok(game)
    }

    pub fn split(&mut self, isopath: &Path, dstpath: &Path) -> Result<()> {
        // TODO self.num_chunks = Some(num_chunks);
        unimplemented!();
    }

    pub fn remove(&self, gamepath: &Path) -> Result<()> {
        unimplemented!();
    }
}