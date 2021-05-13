use std::ops::Range;

pub struct EnemyFire(pub Range<u16>, u16);

impl EnemyFire {
    pub fn new(speed: Range<u16>) -> Self {
        Self(speed, 0)
    }
}
