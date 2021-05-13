use hecs::World;
use macroquad::prelude::draw_rectangle;

use super::{
    map_renderer::{Map, MapColor},
    physics::{Position, Size},
};

#[derive(Clone)]
pub struct AnimatedMapRenderer(pub Vec<(Map, u32)>, MapColor, u32);

impl AnimatedMapRenderer {
    pub fn new(frames: Vec<(Map, u32)>, color: MapColor) -> Self {
        Self(frames, color, 0)
    }
}

pub fn animated_map_renderer_system(world: &mut World) {
    for (_, (renderer, pos, size)) in
        world.query_mut::<(&mut AnimatedMapRenderer, &Position, &Size)>()
    {
        let frame = {
            let mut sum = 0;
            if let Some(i) = renderer
                .0
                .iter()
                .map(|(_, l)| l)
                .enumerate()
                .map(|(i, v)| {
                    sum += v;
                    (i, sum)
                })
                .find(|(_, x)| x >= &renderer.2)
            {
                renderer.2 += 1;
                i.0
            } else {
                renderer.2 = 0;
                0
            }
        };

        let max_size = (
            renderer.0[frame]
                .0
                .iter()
                .fold(0, |f, a| if a.len() > f { a.len() } else { f }) as f32,
            renderer.0[frame].0.len() as f32,
        );
        let cell_size = (size.0 / max_size.0, size.1 / max_size.1);

        renderer.0[frame]
            .0
            .iter()
            .enumerate()
            .flat_map(|(y, s)| s.chars().enumerate().map(move |(x, c)| (x, y, c)))
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
