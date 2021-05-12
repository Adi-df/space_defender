use hecs::World;
use macroquad::prelude::{draw_rectangle, Color};

use super::physics::{Position, Size};

pub struct RectRenderer(pub Color);

impl RectRenderer {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

pub fn rect_renderer_system(world: &mut World) {
    for (_, (renderer, pos, size)) in world.query_mut::<(&RectRenderer, &Position, &Size)>() {
        draw_rectangle(pos.0, pos.1, size.0, size.1, renderer.0);
    }
}
