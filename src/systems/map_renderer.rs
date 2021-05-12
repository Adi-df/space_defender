use std::{collections::HashMap, ops::Deref};

use hecs::World;
use macroquad::prelude::{draw_rectangle, Color};

use super::physics::{Position, Size};

pub struct Map(pub Vec<String>);
pub struct MapColor(pub HashMap<char, Color>);
pub struct MapRenderer(pub Map, pub MapColor);

impl Map {
    pub fn new(map: Vec<String>) -> Self {
        Self(map)
    }
}
impl MapColor {
    pub fn new(map_color: HashMap<char, Color>) -> Self {
        Self(map_color)
    }

    pub fn from_vec(vec: Vec<(char, Color)>) -> Self {
        let mut map = HashMap::new();
        vec.into_iter().for_each(|(char, color)| {
            map.insert(char, color);
        });
        Self(map)
    }
}
impl MapRenderer {
    pub fn new(map: Map, map_color: MapColor) -> Self {
        Self(map, map_color)
    }
}

// From trait
impl From<HashMap<char, Color>> for MapColor {
    fn from(hash: HashMap<char, Color>) -> Self {
        Self(hash)
    }
}
impl From<Vec<(char, Color)>> for MapColor {
    fn from(vec: Vec<(char, Color)>) -> Self {
        Self::from_vec(vec)
    }
}

impl From<Vec<&str>> for Map {
    fn from(vec: Vec<&str>) -> Self {
        Self(vec.into_iter().map(|l| l.to_owned()).collect())
    }
}

// Deref Traits
impl Deref for MapColor {
    type Target = HashMap<char, Color>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for Map {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Map renderer system
pub fn map_renderer_system(world: &mut World) {
    for (_, (renderer, pos, size)) in world.query_mut::<(&MapRenderer, &Position, &Size)>() {
        let max_size = (
            renderer
                .0
                .iter()
                .fold(0, |f, a| if a.len() > f { a.len() } else { f }) as f32,
            renderer.0.len() as f32,
        );
        let cell_size = (size.0 / max_size.0, size.1 / max_size.1);

        renderer
            .0
            .iter()
            .enumerate()
            .flat_map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .map(move |(x, c)| (x, y, c))
            })
            .map(|(x, y, c)| (x as f32, y as f32, *renderer.1.get(&c).unwrap()))
            .for_each(|(x, y, c)| {
                draw_rectangle(
                    pos.0 + x * cell_size.0,
                    pos.1 + y * cell_size.1,
                    cell_size.0,
                    cell_size.1,
                    c,
                )
            });
    }
}
