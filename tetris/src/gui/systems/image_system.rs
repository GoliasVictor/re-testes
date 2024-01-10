//! Structs and processes for draw a image in the screen

use std::rc::Rc;

use glium::{
    implement_vertex, texture::SrgbTexture2d, uniform, uniforms, Display, Frame, Program, Surface,
};

use crate::{
    gui::{transform, Rect},
    vec2,
};

/// Represents a vertex in an image.
///
/// It contains the position and texture coordinates of a verte
#[derive(Copy, Clone)]
pub struct VertexImage {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(VertexImage, position, tex_coords);
/// Represents an image object that can be drawn on the screen.
///
pub struct ImageObject {
    /// The ragion of the screen to draw
    pub region: Rect,
    /// The texture of the image to draw
    pub texture: Rc<SrgbTexture2d>,
}

impl ImageObject {
    fn to_vertex_buffer(&self) -> Vec<VertexImage> {
        let Rect { center, size } = self.region;
        vec![
            VertexImage {
                position: (center - size / 2.).into(),
                tex_coords: [0.0, 0.0],
            },
            VertexImage {
                position: (center + vec2!(size.x, -size.y) / 2.).into(),
                tex_coords: [1.0, 0.0],
            },
            VertexImage {
                position: (center + size / 2.).into(),
                tex_coords: [1.0, 1.0],
            },
            VertexImage {
                position: (center + size / 2.).into(),
                tex_coords: [1.0, 1.0],
            },
            VertexImage {
                position: (center + vec2!(-size.x, size.y) / 2.).into(),
                tex_coords: [0.0, 1.0],
            },
            VertexImage {
                position: (center - size / 2.).into(),
                tex_coords: [0.0, 0.0],
            },
        ]
    }
}

/// Store the program and logic to draw a image in the screen.
///
/// It contains the program for drawing images.
pub struct ImageSystem {
    /// The shaders for drawing images
    pub program: Program,
}
impl ImageSystem {
    /// Create a new Image system
    ///
    /// Load the shaders and create the Image System
    pub fn new(display: &Display) -> ImageSystem {
        let vertex_shader_src: &str = include_str!("./shaders/image.vert");
        let fragment_shader_src = include_str!("./shaders/image.frag");
        let program =
            glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();
        ImageSystem { program }
    }
    /// Draws an image object on the screen.
    ///
    /// It calculates the uniforms for drawing the image object, creates a vertex buffer, and draws the image object using the program of the image system.
    pub fn draw(
        &self,
        target: &mut Frame,
        display: &Display,
        camera_transform: transform::Transform,
        object: &ImageObject,
    ) {
        let behavior: uniforms::SamplerBehavior = uniforms::SamplerBehavior {
            minify_filter: uniforms::MinifySamplerFilter::NearestMipmapLinear,
            magnify_filter: uniforms::MagnifySamplerFilter::Nearest,
            ..Default::default()
        };
        let uniforms = uniform! {
            matrix:  camera_transform.0,
            tex:  uniforms::Sampler(&*object.texture, behavior),
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
