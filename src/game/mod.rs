mod crc;
mod serial;

use std::path::Path;
use std::io::prelude::*;
use std::io::Result;
use std::fs::{File, metadata, remove_file};

const CHUNK_SIZE: u64 = 1_073_741_824;

pub struct Game {
    crc_name: String,
    pub opl_name: String,
    pub serial: String,
    pub num_chunks: Option<i32>
}

impl Game {
    pub fn from_iso(isopath: &Path, opl_name: String) -> Result<Game> {
        let crc_name = crc::get_game_name_crc(&opl_name);
        let serial = serial::get_serial_from_iso(isopath)?;

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
        let meta = metadata(isopath)?;
        let mut file = File::open(isopath)?;

        let n_chunksf = meta.len() as f64 / CHUNK_SIZE as f64;
        let n_chunks = n_chunksf.ceil() as i32;
        let mut offset: u64 = 0;

        for chunk in 0..n_chunks {
            print!("Creating chunk {} of {}...", chunk + 1, n_chunks);

            // even largest ps2 game should not be over 9 chunks
            let chunkname = format!("ul.{}.{}.0{}", &self.crc_name, &self.serial, chunk);
            let chunkpath = dstpath.join(Path::new(&chunkname));
            let mut dst = File::create(chunkpath)?;

            file.seek(std::io::SeekFrom::Start(offset))?;
            let mut src = file.take(CHUNK_SIZE);
            std::io::copy(&mut src, &mut dst)?;
            file = src.into_inner();

            offset += CHUNK_SIZE;
            println!("Done.");
        }

        self.num_chunks = Some(n_chunks);
        Ok(())
    }

    pub fn delete_chunks(&self, ulpath: &Path) -> Result<()> {
        println!("Deleting {}", &self.opl_name);

        for chunk in 0..self.num_chunks.unwrap() {
            print!("Deleting chunk {} of {}...", chunk + 1, self.num_chunks.unwrap());
            let chunkname = format!("ul.{}.{}.0{}", &self.crc_name, &self.serial, chunk);
            let chunkpath = ulpath.join(Path::new(&chunkname));
            remove_file(chunkpath)?;
            println!("Done");
        }

        Ok(())
    }
}