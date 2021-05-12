use std::collections::HashMap;

use hecs::World;
use macroquad::prelude::*;

use crate::exit_modes::ExitMode;
use crate::systems::{map_renderer, physics, player_control};

pub async fn game() -> ExitMode {
    let mut world = World::new();

    let mut map_color = HashMap::new();
    map_color.insert(' ', Color::from_rgba(0, 0, 0, 0));
    map_color.insert('#', RED);

    let player = world.spawn((
        player_control::PlayerControl(10.),
        map_renderer::MapRenderer(
            vec!["  #  ", " ### ", "#####"].into(),
            vec![(' ', Color::from_rgba(0, 0, 0, 0)), ('#', RED)].into(),
        ),
        physics::Position(screen_width() / 2. - 15., screen_height() - 50.),
        physics::Size(30., 30.),
        physics::Velocity(0., 0.),
    ));

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        player_control::player_system(&mut world, &player);
        physics::velocity_system(&mut world);
        map_renderer::map_renderer_system(&mut world);

        next_frame().await;
    }

    ExitMode::Quit
}
