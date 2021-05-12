use std::{collections::HashMap, ops::Deref};

use hecs::World;
use macroquad::prelude::Color;

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
            map.insert(char, color).unwrap();
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