use hecs::{Entity, World};
use macroquad::prelude::{is_key_down, screen_width, KeyCode};

use super::physics::{Position, Size, Velocity};

pub struct PlayerControl(pub f32);

impl PlayerControl {
    pub fn new(speed: f32) -> Self {
        Self(speed)
    }
}

pub fn player_system(world: &mut World, player: &Entity) {
    let (speed, pos, size, vel) = world
        .query_one_mut::<(&PlayerControl, &Position, &Size, &mut Velocity)>(*player)
        .unwrap();
    if is_key_down(KeyCode::Left) && pos.0 > 0. {
        vel.0 = -speed.0;
    } else if is_key_down(KeyCode::Right) && pos.0 + size.0 < screen_width() {
        vel.0 = speed.0;
    } else {
        vel.0 = 0.;
    }
}
