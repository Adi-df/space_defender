pub struct FireControl(pub u32, u32);

impl FireControl {
    pub fn new(cooldown: u32) -> Self {
        Self(cooldown, 0)
    }
}
