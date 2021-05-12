mod exit_modes;
mod game;
mod mainmenu;
mod systems;

use crate::exit_modes::ExitMode;

#[macroquad::main("Space Defender")]
async fn main() {
    if mainmenu::menu().await == ExitMode::Quit {
        return;
    };

    loop {
        if game::game().await == ExitMode::Quit {
            break;
        }
    }
}
