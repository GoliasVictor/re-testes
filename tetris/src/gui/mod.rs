mod square;
mod transform;

use glium::{
    glutin::{self, event_loop, window},
    Display, Frame, Program, Surface, uniform, implement_vertex,
};

pub use square::Square;
pub use transform::*;
use crate::vector2;
use crate::vec2;
use crate::vector2::Vec2;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);


pub struct Camera {
    pub world: Square,
    pub target: Square,
}
impl Camera {
    pub fn get_transformation(&self, pos: Vec2) -> transform::Transform {
        let pos = pos - self.world.center;
        let scale = Vec2 {
            x: self.target.size.x / self.world.size.x,
            y: self.target.size.y / self.world.size.y,
        };
        transform::Transform::default()
            .translate(pos)
            .scale(scale)
            .translate(self.target.center)
    }
}

pub struct Interface {
    pub display: Display,
    pub program: Program,
    pub camera: Camera,
}
impl Interface {
    fn create_display(event_loop: &event_loop::EventLoop<()>) -> Display {
        let wb: window::WindowBuilder = window::WindowBuilder::new()
            .with_decorations(true)
            .with_maximized(true)
            .with_resizable(true)
            .with_title("hello")
            .with_always_on_top(false);
        let cb = glutin::ContextBuilder::new();
        Display::new(wb, cb, event_loop).unwrap()
    }

    pub fn create(event_loop: &event_loop::EventLoop<()>) -> Interface {
        let display = Self::create_display(event_loop);

        let program = {
            let vertex_shader_src: &str = include_str!("shader.vert");
            let fragment_shader_src = include_str!("shader.frag");
            glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
                .unwrap()
        };

        let dims = display.get_framebuffer_dimensions();
        let camera = Camera {
            world: Square {
                center: vec2!(0., 0.),
                size: vec2!(100., 100.),
            },
            target: Square {
                center: vec2!(0., 0.),
                size: Vec2 {
                    x: (dims.1 as f32) / (dims.0 as f32),
                    y: 1.,
                },
            },
        };

        Interface {
            camera,
            display,
            program,
        }
    }
    pub fn draw(&self) -> Canvas {
        Canvas {
            target: self.display.draw(),
            interface: self,
        }
    }
}

pub struct Canvas<'a> {
    pub target: Frame,
    pub interface: &'a Interface,
}

impl<'a> Canvas<'a> {
    pub fn draw_obj(&mut self, obj: &Object) {
        let camera: transform::Transform = self.interface.camera.get_transformation(vector2::ZERO);
        let uniforms = uniform! {
            matrix:  camera.0,
        };

        self.target
            .draw(
                &glium::VertexBuffer::new(&self.interface.display, &obj.to_vertex_buffer()).unwrap(),
                glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &self.interface.program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
    pub fn draw_buffer<T: Iterator<Item = Object>>(&mut self, buffer : T ){
        let vertex_buffer = buffer.flat_map(|o| o.to_vertex_buffer()).collect::<Vec<Vertex>>();

        let camera: transform::Transform = self.interface.camera.get_transformation(vector2::ZERO);
        let uniforms = uniform! {
            matrix:  camera.0,
        };

        self.target
            .draw(
                &glium::VertexBuffer::new(&self.interface.display, &vertex_buffer).unwrap(),
                glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &self.interface.program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }
}
pub struct Object {
    pub format: Square,
    pub color: [f32; 3],
}
impl Object {
    fn to_vertex_buffer(&self) -> Vec<Vertex> {
        let Square { center, size } = self.format;
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
