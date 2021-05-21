#[derive(Clone)]
pub struct Countdown(pub u32);

impl Countdown {
    pub fn new(countdown: u32) -> Self {
        Self(countdown)
    }
}