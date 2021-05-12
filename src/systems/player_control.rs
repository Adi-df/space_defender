pub struct PlayerControl(f32);

impl PlayerControl {
    pub fn new(speed: f32) -> Self {
        Self(speed)
    }
}

