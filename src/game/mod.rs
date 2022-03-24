mod crc;
mod iso;

use crate::game::iso::{Chunk, GameChunk, ISOChunk};

use std::fs::{read_dir, remove_file, File};
use std::io::prelude::*;
use std::io::{copy, stdout, Result, SeekFrom};
use std::io::{Error, ErrorKind};
use std::path::Path;

const CHUNK_SIZE: u64 = 1_073_741_824;

pub struct Game {
    pub opl_name: String,
    crc_name: String,
    chunks: Vec<Box<dyn Chunk>>,
}

impl Game {
    pub fn from_iso(path: &Path, opl_name: String) -> Self {
        let crc_name = crc::get_game_name_crc(&opl_name);
        let chunk = ISOChunk::from(path.to_path_buf());
        let chunks: Vec<Box<dyn Chunk>> = vec![Box::new(chunk)];

        Game {
            opl_name,
            crc_name,
            chunks,
        }
    }

    pub fn from_config(path: &Path, opl_name: String) -> Self {
        let crc_name = crc::get_game_name_crc(&opl_name);
        let chunks = list_game_chunks(path, &crc_name)
            .unwrap_or_default()
            .iter()
            .map(|c| {
                let p = path.to_path_buf().join(c);
                Box::new(GameChunk::from(p)) as Box<dyn Chunk>
            })
            .collect::<Vec<Box<dyn Chunk>>>();

        Game {
            opl_name,
            crc_name,
            chunks,
        }
    }

    pub fn create_chunks(&mut self, dstpath: &Path) -> Result<()> {
        let image = self.chunks.pop().ok_or(ErrorKind::NotFound)?;
        let mut file = File::open(image.path())?;
        let mut offset: u64 = 0;

        if !self.chunks.is_empty() {
            // Any elements left would mean we are splitting already split image.
            return Err(Error::from(ErrorKind::InvalidInput));
        }

        // TODO Simplify when https://github.com/rust-lang/rust/issues/88581 closes.
        let mut n_chunks = file.metadata().unwrap().len() / CHUNK_SIZE;
        if file.metadata().unwrap().len() % CHUNK_SIZE > 0 {
            n_chunks += 1;
        }

        for chunk in 0..n_chunks {
            print!("Creating chunk {} of {}...", chunk + 1, n_chunks);
            stdout().flush().unwrap();

            // Even largest PS2 game should not be over 9 chunks.
            let chunkname = format!("ul.{}.{}.0{}", &self.crc_name, &image.serial()?, chunk);
            let chunkpath = dstpath.join(&chunkname);
            let mut dst = File::create(&chunkpath)?;

            file.seek(SeekFrom::Start(offset))?;
            let mut src = file.take(CHUNK_SIZE);
            copy(&mut src, &mut dst)?;
            file = src.into_inner();

            self.chunks.push(Box::new(GameChunk::from(chunkpath)));
            offset += CHUNK_SIZE;
            println!("Done.");
        }

        Ok(())
    }

    pub fn delete_chunks(&self) -> Result<()> {
        println!("Deleting {}", &self.opl_name);

        for (num, chunk) in self.chunks.iter().enumerate() {
            print!("Deleting chunk {} of {}...", num + 1, self.chunks.len());
            stdout().flush().unwrap();
            remove_file(chunk.path())?;
            println!("Done");
        }

        Ok(())
    }

    pub fn serial(&self) -> String {
        self.chunks
            .get(0)
            .and_then(|c| c.serial().ok())
            .ok_or(ErrorKind::InvalidData)
            .unwrap_or_else(|_| String::from("NOT FOUND"))
    }

    pub fn num_chunks(&self) -> u8 {
        self.chunks.len() as u8
    }

    pub fn formatted_size(&self) -> String {
        let total_size: u64 = self
            .chunks
            .iter()
            .map(|c| c.size().unwrap_or(0))
            .collect::<Vec<u64>>()
            .iter()
            .sum();

        let size_gb = total_size as f64 / 1_000_000_000.0;
        format!("{:.2}GB", size_gb)
    }
}

fn list_game_chunks(path: &Path, crc_name: &str) -> Result<Vec<String>> {
    let chunks = read_dir(path)?
        .map(|res| res.unwrap().file_name().into_string().unwrap())
        .filter(|n| n.contains(crc_name))
        .collect::<Vec<_>>();

    if chunks.is_empty() {
        return Err(Error::from(ErrorKind::NotFound));
    }

    Ok(chunks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_list_game_chunks() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        let chunks = list_game_chunks(&path, "84BA9D95").unwrap();
        assert_eq!(chunks.len(), 2);
    }

    #[test]
    fn test_list_game_chunks_file_not_found() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources");
        let chunks = list_game_chunks(&path, "00000000");
        assert!(chunks.is_err());
    }
}
