use macroquad::prelude::Color;

struct RectRenderer(Color);

impl RectRenderer {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}