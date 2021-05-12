mod mainmenu;
mod exit_modes;
mod game;

use crate::exit_modes::ExitMode;

#[macroquad::main("Space Defender")]
async fn main() {
    if  mainmenu::menu().await == ExitMode::Quit { return; };

    loop {
        if game::game().await == ExitMode::Quit { break; }
    }
}
