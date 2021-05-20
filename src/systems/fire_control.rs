use hecs::{Entity, World};
use macroquad::prelude::{is_key_pressed, KeyCode, GREEN};

use super::{
    bullet,
    physics::{self, Position, Size},
    rect_renderer,
};

pub struct FireControl(pub u32, u32);

impl FireControl {
    pub fn new(cooldown: u32) -> Self {
        Self(cooldown, 0)
    }
}

pub fn fire_control_system(world: &mut World, player: &Entity) {
    if let Some(pos) = {
        let (fire, pos, size) = world
            .query_one_mut::<(&mut FireControl, &Position, &Size)>(*player)
            .unwrap();
        if fire.1 > 0 {
            fire.1 -= 1
        };

        if is_key_pressed(KeyCode::Space) && fire.1 == 0 {
            Some(physics::Position::new(pos.0 + size.0 / 2., pos.1 - 6.))
        } else {
            None
        }
    } {
        world.spawn((
            bullet::Bullet::new("Player Bullet"),
            rect_renderer::RectRenderer::new(GREEN),
            pos,
            physics::Size(5., 5.),
            physics::Velocity(0., -8.),
        ));
    }
}
