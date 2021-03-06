mod parser;
mod status;
mod table;

use crate::game::Game;
use crate::ul::status::GameStatus;

use std::fs::{write, File};
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

const UL_GAME_SIZE: usize = 64;
const UL_GAME_NAME_SIZE: usize = 32;
const UL_GAME_CHUNK_COUNT: usize = 47;
const UL_SERIAL_SIZE: usize = 12;
const UL_EMPTY_SIZE: usize = 4;
const UL_NAME_EXT_SIZE: usize = 10;
const SCEC_DVD_MEDIA_TYPE: u8 = 0x14;
const USBEXTREME_MAGIC: u8 = 0x08;

macro_rules! strvec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

pub struct Ulcfg {
    games: Vec<Game>,
    states: Vec<GameStatus>,
}

impl Ulcfg {
    pub fn new() -> Self {
        let games: Vec<Game> = Vec::new();
        let states: Vec<GameStatus> = Vec::new();
        Ulcfg { games, states }
    }

    pub fn load(path: &Path) -> Result<Self> {
        let mut file = File::open(&path)?;
        let mut games: Vec<Game> = Vec::new();
        let mut states: Vec<GameStatus> = Vec::new();

        loop {
            let mut handle = file.take(UL_GAME_SIZE as u64);
            let mut buffer = vec![0x00; UL_GAME_SIZE];

            if handle.read(&mut buffer)? < buffer.len() {
                break;
            }

            file = handle.into_inner();
            let opl_name = parser::parse_to_string(&buffer, 0, UL_GAME_NAME_SIZE);
            let game = Game::from_config(path.parent().unwrap(), opl_name);

            let mut state = GameStatus::Good;
            if game.num_chunks() == 0 {
                state = GameStatus::from("NO DATA");
            } else if game.num_chunks() != buffer[UL_GAME_CHUNK_COUNT] {
                state = GameStatus::from("LOST DATA");
            }

            games.push(game);
            states.push(state);
        }

        Ok(Ulcfg { games, states })
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let mut ulbuff: Vec<u8> = Vec::new();

        for entry in &self.games {
            // first 32 bytes are padded OPL game name
            let game_name_bytes = parser::compose_from_str(&entry.opl_name, UL_GAME_NAME_SIZE);
            ulbuff.extend_from_slice(&game_name_bytes);

            // next 15 bytes are serial with `ul.` prefix and padding
            ulbuff.extend_from_slice(&[0x75, 0x6c, 0x2e]);
            let serial_bytes = parser::compose_from_str(&entry.serial(), UL_SERIAL_SIZE);
            ulbuff.extend_from_slice(&serial_bytes);

            // next byte is number of game chunks
            ulbuff.push(entry.num_chunks());

            // last 16 bytes are just constants
            ulbuff.push(SCEC_DVD_MEDIA_TYPE);
            ulbuff.extend_from_slice(&[0x00; UL_EMPTY_SIZE]);
            ulbuff.push(USBEXTREME_MAGIC);
            ulbuff.extend_from_slice(&[0x00; UL_NAME_EXT_SIZE]);
        }

        write(path, &ulbuff)?;
        Ok(())
    }

    pub fn list_games(&self) {
        let col_names = strvec!["Index", "Name", "Serial", "Size", "Status"];
        let col_sizes = vec![5, UL_GAME_NAME_SIZE, UL_SERIAL_SIZE, 6, 10];
        let hline = table::make_hline(&col_sizes);
        let header = table::make_row(col_names, &col_sizes);

        println!("{}", hline);
        println!("{}", header);
        println!("{}", hline);

        let it = self.games.iter().zip(self.states.iter());
        for (pos, (game, state)) in it.enumerate() {
            let contents = vec![
                pos.to_string(),
                String::from(&game.opl_name),
                game.serial(),
                game.formatted_size(),
                format!("{}", state),
            ];
            let row = table::make_row(contents, &col_sizes);
            println!("{}", row);
        }

        println!("{}", hline);
    }

    pub fn add_game(&mut self, isopath: &Path, dstpath: &Path, opl_name: String) -> Result<()> {
        let mut game = Game::from_iso(isopath, opl_name);
        // TODO Cleanup if create_chunks failed?
        game.create_chunks(dstpath)?;
        self.games.push(game);

        Ok(())
    }

    pub fn delete_game_by_name(&mut self, name: &str) -> Result<()> {
        for (index, game) in self.games.iter().enumerate() {
            if game.opl_name == name {
                self.delete_game(index)?;
                return Ok(());
            }
        }

        Err(Error::from(ErrorKind::InvalidInput))
    }

    pub fn delete_game_by_index(&mut self, index: usize) -> Result<()> {
        if index >= self.games.len() {
            return Err(Error::from(ErrorKind::InvalidInput));
        }

        self.delete_game(index)?;
        Ok(())
    }

    fn delete_game(&mut self, index: usize) -> Result<()> {
        let game = self.games.remove(index);
        self.states.remove(index);
        game.delete_chunks()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_ulcfg_load_failed() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/fake.ul.cfg");
        let ulcfg = Ulcfg::load(&path);
        assert!(ulcfg.is_err());
    }

    #[test]
    fn test_ulcfg_load_success() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/ul.cfg");
        let ulcfg = Ulcfg::load(&path);
        assert!(ulcfg.is_ok());
    }
}
