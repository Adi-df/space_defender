use super::map_renderer::{Map, MapColor};

pub struct AnimatedMapRenderer(pub Vec<(Map, u32)>, MapColor, u32);

impl AnimatedMapRenderer {
    pub fn new(frames: Vec<(Map, u32)>, color: MapColor) -> Self {
        Self(frames, color, 0)
    }
}
