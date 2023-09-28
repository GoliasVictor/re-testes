mod transform;
pub mod interface;

use glium::implement_vertex;

pub use transform::*;
use crate::{vec2, vector2::Vec2};

#[derive(Debug)]
pub struct Rect {
    pub size: Vec2,
    pub center: Vec2,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

#[derive(Debug)]
pub struct Object {
    pub format: Rect,
    pub color: [f32; 3],
}
impl Object {
    fn to_vertex_buffer(&self) -> Vec<Vertex> {
        let Rect { center, size } = self.format;
        vec![
            Vertex {
                position: (center - size / 2.).into(),
                color: self.color.clone(),
            },
            Vertex {
                position: (center + vec2!(size.x, -size.y) / 2.).into(),
                color: self.color.clone(),
            },
            Vertex {
                position: (center + size / 2.).into(),
                color: self.color.clone(),
            },
            Vertex {
                position: (center + size / 2.).into(),
                color: self.color.clone(),
            },
            Vertex {
                position: (center + vec2!(-size.x, size.y) / 2.).into(),
                color: self.color.clone(),
            },
            Vertex {
                position: (center - size / 2.).into(),
                color: self.color.clone(),
            },
        ]
    }
}
