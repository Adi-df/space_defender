use hecs::{Entity, World};

#[derive(Clone)]
pub struct Countdown(pub u32);

impl Countdown {
    pub fn new(countdown: u32) -> Self {
        Self(countdown)
    }
}

pub fn countdown_system(world: &mut World) {
    let outs = world
        .query_mut::<&mut Countdown>()
        .into_iter()
        .filter_map(|(e, countdown)| {
            countdown.0 -= 1;

            if countdown.0 == 0 {
                Some(e)
            } else {
                None
            }
        })
        .collect::<Vec<Entity>>();

    outs.into_iter().for_each(|out| {
        world.despawn(out).unwrap();
    });
}
