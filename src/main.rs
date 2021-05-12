use macroquad::prelude::*;

mod mainmenu;
mod exit_modes;

use crate::exit_modes::ExitMode;

#[macroquad::main("Space Defender")]
async fn main() {
    if  mainmenu::menu().await == ExitMode::Quit { return; };
}
