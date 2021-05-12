use hecs::World;

use crate::exit_modes::ExitMode;

pub async fn game() -> ExitMode {
    let mut world = World::new();

    ExitMode::Quit
}