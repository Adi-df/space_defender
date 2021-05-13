use std::sync::Arc;

use hecs::{Entity, World};

#[derive(Clone)]
pub struct Life {
    pub life: u16,
    death: Arc<Box<dyn Send + Sync + 'static + Fn(&mut World, &Entity)>>,
}

impl Life {
    pub fn new(l: u16, d: Box<dyn Send + Sync + 'static + Fn(&mut World, &Entity)>) -> Self {
        Self {
            life: l,
            death: Arc::new(d),
        }
    }
}

pub fn life_system(world: &mut World) {
    let died = world
        .query_mut::<&Life>()
        .into_iter()
        .filter_map(|(e, l)| if l.life == 0 { Some(e) } else { None })
        .collect::<Vec<Entity>>();

    died.into_iter().for_each(|d| {
        let l = &*world.query_one_mut::<&Life>(d).unwrap().death.clone();
        l(world, &d);
        world.despawn(d).unwrap();
    })
}
