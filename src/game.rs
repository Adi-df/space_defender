use hecs::World;
use macroquad::prelude::*;

use crate::exit_modes::ExitMode;
use crate::systems::physics;

pub async fn game() -> ExitMode {
    let mut world = World::new();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }

    ExitMode::Quit
}
