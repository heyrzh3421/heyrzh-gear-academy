#![no_std]

use gstd::{debug, exec, msg};
use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;


#[no_mangle]
pub extern "C" fn init() {

}

#[no_mangle]
pub extern "C" fn handle() {

}

#[no_mangle]
extern "C" fn state() {

}
