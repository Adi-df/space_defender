use hecs::World;

use super::physics;

pub struct PathFollower(pub f32, pub Vec<(f32, f32)>, usize);

impl PathFollower {
    pub fn new(speed: f32, path: Vec<(f32, f32)>) -> Self {
        Self(speed, path, 0)
    }
}

pub fn path_follower_system(world: &mut World) {
    for (_, (path, pos, vel)) in world.query_mut::<(
        &mut PathFollower,
        &physics::Position,
        &mut physics::Velocity,
    )>() {
        let target = path.1[path.2];
        if (target.0 - pos.0).abs() < 0. && (target.1 - pos.1).abs() < 0. {
            *vel = physics::Velocity::new(0., 0.);
            path.2 += 1;
            if path.2 == path.1.len() {
                path.2 = 0;
            }
        }

        if vel.0 == 0. && vel.1 == 0. {
            *vel = physics::Velocity::new((target.0 - pos.0) / path.0, (target.1 - pos.1) / path.0)
        }
    }
}
