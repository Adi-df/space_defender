use macroquad::{prelude::*, rand::srand};

mod exit_modes;
mod game;
mod mainmenu;
mod systems;

use crate::exit_modes::ExitMode;

#[macroquad::main("Space Defender")]
async fn main() {
    srand((get_time() * 100000.).floor() as u64);

    if mainmenu::menu().await == ExitMode::Quit {
        return;
    };

    loop {
        if game::game().await == ExitMode::Quit {
            break;
        }
    }
}
