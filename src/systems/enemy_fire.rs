use std::ops::Range;

use hecs::World;
use macroquad::{prelude::RED, rand::gen_range};

use super::{
    bullet::Bullet,
    physics::{Position, Size, Velocity},
    rect_renderer::RectRenderer,
};

pub struct EnemyFire(pub Range<u16>, u16);

impl EnemyFire {
    pub fn new(speed: Range<u16>) -> Self {
        let Range {
            start: rstart,
            end: rend,
        } = speed.clone();
        Self(
            speed,
            gen_range(
                if rstart > 0 { rstart } else { 1 } as u32,
                if rend > 0 { rend } else { 1 } as u32,
            ) as u16,
        )
    }
}

pub fn enemy_fire_system(world: &mut World) {
    // Decrease countdown
    world
        .query_mut::<&mut EnemyFire>()
        .into_iter()
        .for_each(|(_, f)| {
            f.1 -= 1;
        });

    // Fire
    let fire = world
        .query_mut::<(&mut EnemyFire, &Position, &Size)>()
        .into_iter()
        .filter_map(|(_, (f, p, s))| if f.1 == 0 { Some((f, p, s)) } else { None })
        .map(|(f, pos, size)| {
            f.1 = gen_range(f.0.start as u32, f.0.end as u32) as u16;

            (
                Bullet::new(),
                RectRenderer::new(RED),
                Position::new(pos.0 + size.0 / 2., pos.1 + size.1),
                Size(5., 5.),
                Velocity(0., 5.),
            )
        })
        .collect::<Vec<(Bullet, RectRenderer, Position, Size, Velocity)>>();

    fire.into_iter().for_each(|b| {
        world.spawn(b);
    });
}
