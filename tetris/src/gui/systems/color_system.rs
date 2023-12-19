//! Structs and process to draw a color in the screen

use glium::{implement_vertex, uniform, Display, Frame, Program, Surface};

use crate::{
    gui::{transform, Rect},
    vec2,
};

/// A struct representing a vertex.
#[derive(Copy, Clone)]
pub struct VertexColor {
    /// The position of the vertex.
    position: [f32; 2],
    /// The color of the vertex.
    color: [f32; 3],
}

implement_vertex!(VertexColor, position, color);

/// Store the program and logic to draw an object of solid color in the screen.
pub struct ColorSystem {
    program: Program,
}
impl ColorSystem {
    /// Create a a new Color Sytem
    ///
    /// Load the shaders and initialize the color System
    pub fn new(display: &Display) -> ColorSystem {
        let vertex_shader_src = include_str!("./shaders/solid_color.vert");
        let fragment_shader_src = include_str!("./shaders/solid_color.frag");
        let program =
            glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();
        ColorSystem { program }
    }
    /// Draw a object of solid color in the frame
    pub fn draw(
        &self,
        target: &mut Frame,
        display: &Display,
        camera_transform: transform::Transform,
        object: &SolidColorObject,
    ) {
        let uniforms = uniform! {
            matrix: camera_transform.0,
        };

        target
            .draw(
                &glium::VertexBuffer::new(display, &object.to_vertex_buffer()).unwrap(),
                glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}

/// A struct representing an object in the interface, with color and format.
pub struct SolidColorObject {
    /// The format of the object.
    pub format: Rect,
    /// The color of the object.
    pub color: [f32; 3],
}

impl SolidColorObject {
    fn to_vertex_buffer(&self) -> Vec<VertexColor> {
        let Rect { center, size } = self.format;
        vec![
            VertexColor {
                position: (center - size / 2.).into(),
                color: self.color,
            },
            VertexColor {
                position: (center + vec2!(size.x, -size.y) / 2.).into(),
                color: self.color,
            },
            VertexColor {
                position: (center + size / 2.).into(),
                color: self.color,
            },
            VertexColor {
                position: (center + size / 2.).into(),
                color: self.color,
            },
            VertexColor {
                position: (center + vec2!(-size.x, size.y) / 2.).into(),
                color: self.color,
            },
            VertexColor {
                position: (center - size / 2.).into(),
                color: self.color,
            },
        ]
    }
}
