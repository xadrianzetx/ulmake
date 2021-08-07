mod parser;

use crate::game::Game;

use std::path::Path;
use std::fs::read;
use std::io::Result;

const UL_GAME_SIZE: usize = 64;
const UL_GAME_NAME_START: usize = 0;
const UL_GAME_NAME_END: usize = 32;
const UL_SERIAL_START: usize = 35;
const UL_SERIAL_END: usize = 47;

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
            
            let game = Game::from_config(opl_name, serial, num_chunks).unwrap();
            game_list.push(game);

            start_index += UL_GAME_SIZE;
        }
        
        let ulcfg = Ulcfg { game_list: game_list };
        Ok(ulcfg)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        unimplemented!();
    }

    pub fn list_games(&self) {
        // TODO make pretty print
        println!("Name|Serial");
        for game in &self.game_list {
            println!("{}|{}", game.opl_name, game.serial);
        }
    }

    pub fn add_game(&mut self, isopath: &Path, dstpath: &Path, opl_name: String) -> Result<()> {
        let mut game = Game::from_iso(isopath, opl_name)?;
        game.split(isopath, dstpath)?;
        self.game_list.push(game);

        Ok(())
    }

    pub fn remove_game(&mut self) -> Result<()> {
        // TODO must be able to remove by name
        // or by index
        unimplemented!();
    }
}