use hecs::{Entity, World};
use macroquad::prelude::{screen_height, screen_width};

use super::physics::Position;

pub struct Bullet;

impl Bullet {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn bullet_system(world: &mut World) {
    let out_bullets = world
        .query_mut::<(&Bullet, &Position)>()
        .into_iter()
        .filter_map(|(e, (_, p))| {
            if p.0 < 0. || p.0 > screen_width() || p.1 < 0. || p.1 > screen_height() {
                Some(e)
            } else {
                None
            }
        })
        .collect::<Vec<Entity>>();

    out_bullets
        .into_iter()
        .for_each(|b| world.despawn(b).unwrap());
}
