use glium::{
    glutin::{self, event_loop, window},
    uniform, Display, Frame, Program, Surface,
};

use crate::vec2;
use crate::vector2::Vec2;

use super::{
    transform::{self, *},
    Object, Rect, Vertex,
};

/// `Interface` struct is used to encapsulate the display, program, and camera.
pub struct Interface {
    /// The `display` represents the display window.
    pub display: Display,
    /// The `program` represents the shader program.
    pub program: Program,
    /// The `camera` represents the camera view.
    pub camera: Camera,
}

impl Interface {
    /// Creates a new display for the event loop.
    ///
    /// This function takes a reference to an event loop and creates a new display for it with
    /// certain properties such as decorations, maximized, resizable, title, and always on top.
    ///
    /// # Examples
    ///
    /// ```
    /// let event_loop = event_loop::EventLoop::new();
    /// let display = Interface::create_display(&event_loop);
    /// ```
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

    /// Creates a new `Interface` instance.
    ///
    /// This function takes a reference to an event loop and creates a new `Interface` instance
    /// with a display, program, and camera.
    ///
    /// # Examples
    ///
    /// ```
    /// let event_loop = event_loop::EventLoop::new();
    /// let interface = Interface::create(&event_loop);
    /// ```
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

    /// Draws the interface.
    ///
    /// This function draws the interface on the display and returns a `Canvas` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let event_loop = event_loop::EventLoop::new();
    /// let interface = Interface::create(&event_loop);
    /// let canvas = interface.draw();
    /// ```
    pub fn draw(&self) -> Canvas {
        Canvas {
            target: self.display.draw(),
            interface: self,
        }
    }
}

/// `Canvas` struct is used for drawing objects on the `Interface`.
pub struct Canvas<'a> {
    /// Represents the frame where the objects will be drawn.
    pub target: Frame,
    /// Represents the interface where the objects will be drawn.
    pub interface: &'a Interface,
}

impl<'a> Canvas<'a> {
    /// Draws an object on the canvas.
    ///
    /// This function takes a reference to an object and draws it on the canvas.
    /// It uses the transformation of the camera to determine the position of the object.
    ///
    /// # Examples
    ///
    /// ```    
    /// let event_loop = event_loop::EventLoop::new();
    /// let interface = Interface::create(&event_loop);
    /// let mut canvas = interface.draw();
    /// let object = Object::new(...);
    /// canvas.draw_obj(&object);
    /// ```

    pub fn draw_obj(&mut self, obj: &Object) {
        let camera: transform::Transform = self.interface.camera.get_transformation(Vec2::ZERO);
        let uniforms = uniform! {
            matrix: camera.0,
        };

        self.target
            .draw(
                &glium::VertexBuffer::new(&self.interface.display, &obj.to_vertex_buffer())
                    .unwrap(),
                glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &self.interface.program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }

    /// Draws a buffer of objects on the canvas.
    ///
    /// This function takes an iterator of objects and draws them on the canvas.
    /// It uses the transformation of the camera to determine the position of the objects.
    ///
    /// # Examples
    ///
    /// ```
    /// let event_loop = event_loop::EventLoop::new();
    /// let interface = Interface::create(&event_loop);
    /// let mut canvas = interface.draw();
    /// let objects = vec![Object::new(...), Object::new(...)];
    /// canvas.draw_buffer(objects.into_iter());
    /// ```

    pub fn draw_buffer<T: Iterator<Item = Object>>(&mut self, buffer: T) {
        let vertex_buffer = buffer
            .flat_map(|o| o.to_vertex_buffer())
            .collect::<Vec<Vertex>>();

        let camera: transform::Transform = self.interface.camera.get_transformation(Vec2::ZERO);
        let uniforms = uniform! {
            matrix: camera.0,
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
