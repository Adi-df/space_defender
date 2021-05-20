use std::sync::{Arc, Mutex};

use hecs::{Entity, World};

use super::{bullet, life, physics};

type OnTouchCallback = Box<dyn Send + Sync + 'static + FnMut(&mut World, &Entity)>;
#[derive(Clone)]
pub struct TakeBulletDamage(String, Arc<Mutex<OnTouchCallback>>);

impl TakeBulletDamage {
    pub fn new(tag: String, on_touch: OnTouchCallback) -> Self {
        Self(tag.to_owned(), Arc::new(Mutex::new(on_touch)))
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
        .map(|(e, (tb, _, pos, size))| (e, tb.0.clone(), pos.clone(), size.clone()))
        .collect::<Vec<(Entity, String, physics::Position, physics::Size)>>();

    let bullets = world
        .query_mut::<(&bullet::Bullet, &physics::Position, &physics::Size)>()
        .into_iter()
        .map(|(e, (b, pos, size))| (e, b.0.clone(), pos.clone(), size.clone()))
        .collect::<Vec<(Entity, String, physics::Position, physics::Size)>>();

    let in_square = |p: (f32, f32), r: (f32, f32, f32, f32)| {
        r.0 <= p.0 && p.0 <= r.0 + r.2 && r.1 <= p.1 && p.1 <= r.1 + r.3
    };

    let touched = targetable_entity
        .into_iter()
        .filter_map(|(e, tag, pos, size)| {
            let touching = bullets
                .iter()
                .filter_map(|(b, btag, bpos, bsize)| {
                    if (in_square((bpos.0, bpos.1), (pos.0, pos.1, size.0, size.1))
                        || in_square((bpos.0 + bsize.0, bpos.1), (pos.0, pos.1, size.0, size.1))
                        || in_square((bpos.0, bpos.1 + bsize.1), (pos.0, pos.1, size.0, size.1))
                        || in_square(
                            (bpos.0 + bsize.0, bpos.1 + bsize.1),
                            (pos.0, pos.1, size.0, size.1),
                        ))
                        && tag == *btag
                    {
                        Some(*b)
                    } else {
                        None
                    }
                })
                .collect::<Vec<Entity>>();

            if !touching.is_empty() {
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
                me.0 .1.clone()
            };
            on_touch.lock().unwrap()(world, &s);
        }
        b.into_iter().for_each(|e| world.despawn(e).unwrap_or(()));
    });
}
