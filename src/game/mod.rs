mod crc;
mod iso;

use std::fs::{metadata, remove_file, File};
use std::io::prelude::*;
use std::io::{copy, stdout, Result, SeekFrom};
use std::path::Path;

const CHUNK_SIZE: u64 = 1_073_741_824;

pub struct Game {
    size: u64,
    crc_name: String,
    pub opl_name: String,
    pub serial: String,
    pub num_chunks: i32,
}

impl Game {
    pub fn from_iso(isopath: &Path, opl_name: String) -> Result<Self> {
        let crc_name = crc::get_game_name_crc(&opl_name);
        let serial = iso::get_serial_from_iso(isopath)?;
        let size = iso::get_size_from_iso(isopath)?;

        let game = Game {
            size,
            opl_name,
            crc_name,
            serial,
            num_chunks: 0,
        };

        Ok(game)
    }

    pub fn from_config(
        chunkpath: &Path,
        opl_name: String,
        serial: String,
        num_chunks: i32,
    ) -> Result<Self> {
        let crc_name = crc::get_game_name_crc(&opl_name);
        let size = iso::get_size_from_chunks(chunkpath, &crc_name)?;

        let game = Game {
            size,
            opl_name,
            crc_name,
            serial,
            num_chunks,
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
            stdout().flush().unwrap();

            // even largest ps2 game should not be over 9 chunks
            let chunkname = format!("ul.{}.{}.0{}", &self.crc_name, &self.serial, chunk);
            let chunkpath = dstpath.join(Path::new(&chunkname));
            let mut dst = File::create(chunkpath)?;

            file.seek(SeekFrom::Start(offset))?;
            let mut src = file.take(CHUNK_SIZE);
            copy(&mut src, &mut dst)?;
            file = src.into_inner();

            offset += CHUNK_SIZE;
            println!("Done.");
        }

        self.num_chunks = n_chunks;
        Ok(())
    }

    pub fn delete_chunks(&self, ulpath: &Path) -> Result<()> {
        println!("Deleting {}", &self.opl_name);

        for chunk in 0..self.num_chunks {
            print!("Deleting chunk {} of {}...", chunk + 1, self.num_chunks);
            stdout().flush().unwrap();

            let chunkname = format!("ul.{}.{}.0{}", &self.crc_name, &self.serial, chunk);
            let chunkpath = ulpath.join(Path::new(&chunkname));

            remove_file(chunkpath)?;
            println!("Done");
        }

        Ok(())
    }

    pub fn iso_size(&self) -> String {
        let size_gb = self.size as f64 / 1_000_000_000.0;
        format!("{:.2}GB", size_gb)
    }
}
