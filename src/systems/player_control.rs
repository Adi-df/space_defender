use hecs::World;
use macroquad::prelude::{KeyCode, is_key_down, screen_width};

use super::physics::{Position, Size, Velocity};

pub struct PlayerControl(pub f32);

impl PlayerControl {
    pub fn new(speed: f32) -> Self {
        Self(speed)
    }
}