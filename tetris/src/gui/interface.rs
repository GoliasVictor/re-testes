

use glium::{
    glutin::{self, event_loop, window},
     Display, Program, Frame, uniform, Surface,
};

use crate::vector2;
use crate::vec2;
use crate::vector2::Vec2;

use super::{transform::{*, self}, Object, Vertex, Rect};


// This structure is used to store interface details
// Display is where to manipulate the screen
// program is where the shaders are
// And the camera is for where the screen is viewed
pub struct Interface {
    pub display: Display,
    pub program: Program,
    pub camera: Camera,
}

impl Interface {
    pub(crate) fn create_display(event_loop: &event_loop::EventLoop<()>) -> Display {
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
            world: Rect {
                center: vec2!(0., 0.),
                size: vec2!(100., 100.),
            },
            target: Rect {
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
