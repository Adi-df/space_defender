use std::sync::Arc;

use hecs::{Entity, World};

use super::{bullet, life, physics};

#[derive(Clone)]
pub struct TakeBulletDamage(Arc<Box<dyn Send + Sync + 'static + Fn(&mut World, &Entity)>>);

impl TakeBulletDamage {
    pub fn new(on_touch: Box<dyn Send + Sync + 'static + Fn(&mut World, &Entity)>) -> Self {
        Self(Arc::new(on_touch))
    }
}

pub fn take_bullet_damage_system(world: &mut World) {
    let targetable_entity = world
        .query_mut::<(
            &TakeBulletDamage,
            &life::Life,
            &physics::Position,
            &physics::Size,
        )>()
        .into_iter()
        .map(|(e, (_, _, pos, size))| (e, pos.clone(), size.clone()))
        .collect::<Vec<(Entity, physics::Position, physics::Size)>>();

    let bullets = world
        .query_mut::<(&bullet::Bullet, &physics::Position, &physics::Size)>()
        .into_iter()
        .map(|(e, (_, pos, size))| (e, pos.clone(), size.clone()))
        .collect::<Vec<(Entity, physics::Position, physics::Size)>>();

    let in_square = |p: (f32, f32), r: (f32, f32, f32, f32)| {
        r.0 <= p.0 && p.0 <= r.0 + r.2 && r.1 <= p.1 && p.1 <= r.1 + r.3
    };

    let touched = targetable_entity
        .into_iter()
        .filter_map(|(e, pos, size)| {
            let touching = bullets
                .iter()
                .filter(|(_, bpos, bsize)| {
                    in_square((bpos.0, bpos.1), (pos.0, pos.1, size.0, size.1))
                        || in_square((bpos.0 + bsize.0, bpos.1), (pos.0, pos.1, size.0, size.1))
                        || in_square((bpos.0, bpos.1 + bsize.1), (pos.0, pos.1, size.0, size.1))
                        || in_square(
                            (bpos.0 + bsize.0, bpos.1 + bsize.1),
                            (pos.0, pos.1, size.0, size.1),
                        )
                })
                .map(|(e, _, _)| e.clone())
                .collect::<Vec<Entity>>();

            if touching.len() > 0 {
                Some((e, touching))
            } else {
                None
            }
        })
        .collect::<Vec<(Entity, Vec<Entity>)>>();

    touched.into_iter().for_each(|(s, b)| {
        {
            let on_touch = {
                let me = world
                    .query_one_mut::<(&TakeBulletDamage, &mut life::Life)>(s)
                    .unwrap();
                me.1.life -= 1;
                me.0 .0.clone()
            };
            on_touch(world, &s);
        }
        b.into_iter().for_each(|e| world.despawn(e).unwrap());
    });
}
