use std::collections::HashMap;

use hecs::World;
use macroquad::prelude::{Color};

pub type MapColor = HashMap<char, Color>;
pub struct Map(pub Vec<String>);
pub struct MapRenderer(pub Map, pub MapColor);

impl Map {
    pub fn new(map: Vec<String>) -> Self {
        Self(map)
    }
}
impl MapRenderer {
    pub fn new(map: Map, map_color: MapColor) -> Self {
        Self(map, map_color)
    }
}