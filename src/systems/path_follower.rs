pub struct PathFollower(pub f32, pub Vec<(f32, f32)>, usize);

impl PathFollower {
    pub fn new(speed: f32, path: Vec<(f32, f32)>) -> Self {
        Self(speed, path, 0)
    }
}
