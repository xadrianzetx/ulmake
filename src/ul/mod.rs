mod parser;

use crate::game::Game;

use std::path::Path;
use std::fs::{read, write};
use std::io::{Result, Error, ErrorKind};

const UL_GAME_SIZE: usize = 64;
const UL_GAME_NAME_START: usize = 0;
const UL_GAME_NAME_END: usize = 32;
const UL_SERIAL_START: usize = 35;
const UL_SERIAL_END: usize = 47;
const UL_EMPTY_SIZE: usize = 4;
const UL_NAME_EXT_SIZE: usize = 10;
const SCEC_DVD_MEDIA_TYPE: u8 = 0x14;
const USBEXTREME_MAGIC: u8 = 0x08;

pub struct Ulcfg {
    game_list: Vec<Game>
}

impl Ulcfg {
    pub fn new() -> Result<Ulcfg> {
        let game_list: Vec<Game> = Vec::new();
        let ulcfg = Ulcfg { game_list: game_list };
        
        Ok(ulcfg)
    }

    pub fn load(path: &Path) -> Result<Ulcfg> {
        let mut game_list: Vec<Game> = Vec::new();
        let ulbuff = read(path)?;
        let num_games = &ulbuff.len() / UL_GAME_SIZE;
        let mut start_index = 0;

        for _ in 0..num_games {
            let gbuff = &ulbuff[start_index..start_index + UL_GAME_SIZE];
            let opl_name = parser::parse_to_string(gbuff, UL_GAME_NAME_START, UL_GAME_NAME_END);
            let serial = parser::parse_to_string(gbuff, UL_SERIAL_START, UL_SERIAL_END);
            let num_chunks = gbuff[UL_SERIAL_END] as i32;
            
            let entry = Game::from_config(opl_name, serial, num_chunks).unwrap();
            game_list.push(entry);

            start_index += UL_GAME_SIZE;
        }
        
        let ulcfg = Ulcfg { game_list: game_list };
        Ok(ulcfg)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let mut ulbuff: Vec<u8> = Vec::new();

        for entry in &self.game_list {
            // first 32 bytes are padded OPL game name
            let game_name_size = UL_GAME_NAME_END - UL_GAME_NAME_START;
            let game_name_bytes = parser::compose_from_str(&entry.opl_name, game_name_size);
            ulbuff.extend_from_slice(&game_name_bytes);

            // next 15 bytes are serial with `ul.` prefix and padding
            ulbuff.extend_from_slice(&vec![0x75, 0x6c, 0x2e]);
            let serial_size = UL_SERIAL_END - UL_SERIAL_START;
            let serial_bytes = parser::compose_from_str(&entry.serial, serial_size);
            ulbuff.extend_from_slice(&serial_bytes);

            // next byte is number of game chunks
            let num_chunks = entry.num_chunks as u8;
            ulbuff.push(num_chunks);

            // last 16 bytes are just constants
            ulbuff.push(SCEC_DVD_MEDIA_TYPE);
            ulbuff.extend_from_slice(&vec![0x00; UL_EMPTY_SIZE]);
            ulbuff.push(USBEXTREME_MAGIC);
            ulbuff.extend_from_slice(&vec![0x00; UL_NAME_EXT_SIZE]);
        }

        write(path, &ulbuff)?;
        Ok(())
    }

    pub fn list_games(&self) {
        // TODO make pretty print
        println!("Index|Name|Serial");
        for (pos, game) in self.game_list.iter().enumerate() {
            println!("{}|{}|{}", pos, game.opl_name, game.serial);
        }
    }

    pub fn add_game(&mut self, isopath: &Path, dstpath: &Path, opl_name: String) -> Result<()> {
        let mut game = Game::from_iso(isopath, opl_name)?;
        game.split(isopath, dstpath)?;
        self.game_list.push(game);

        Ok(())
    }

    pub fn delete_game_by_name(&mut self, name: String, path: &Path) -> Result<()> {
        for (index, game) in self.game_list.iter().enumerate() {
            if game.opl_name == name {
                self.delete_game(index, path)?;
                return Ok(())
            }
        }

        Err(Error::from(ErrorKind::InvalidInput))
    }

    pub fn delete_game_by_index(&mut self, index: usize, path: &Path) -> Result<()> {
        if index >= self.game_list.len() {
            return Err(Error::from(ErrorKind::InvalidInput))
        }

        self.delete_game(index, path)?;
        Ok(())
    }

    fn delete_game(&mut self, index: usize, path: &Path) -> Result<()> {
        self.game_list[index].delete_chunks(path)?;
        self.game_list.remove(index);
        Ok(())
    }
}