//! Module for accessing the interface, with wrappers for communicating with the interface
use std::rc::Rc;

use glium::{
    glutin::{self, event_loop, window},
    texture::Texture2dDataSource,
    Display, Frame,
};

use crate::vec2;
use crate::vector2::Vec2;

use super::{
    systems::color_system::ColorSystem,
    systems::image_system::ImageSystem,
    transform::{self, *},
    ObjectWrapper, Rect,
};

/// `Interface` struct is used to encapsulate the display, and camera.
pub struct Interface {
    /// The `display` represents the display window.
    pub display: Display,
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

        Interface { camera, display }
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
        let color_system = ColorSystem::new(&self.display);
        let image_system = ImageSystem::new(&self.display);
        Canvas {
            target: self.display.draw(),
            interface: self,
            color_system,
            image_system,
        }
    }
    /// Extract the data from datasource and wrap in a [Rc]
    pub fn create_texture<'a, T>(&self, source: T) -> Rc<glium::texture::SrgbTexture2d>
    where
        T: Texture2dDataSource<'a>,
    {
        Rc::new(glium::texture::SrgbTexture2d::new(&self.display, source).unwrap())
    }
}

/// `Canvas` struct is used for drawing objects on the `Interface`.
pub struct Canvas<'a> {
    /// Represents the frame where the objects will be drawn.
    pub target: Frame,
    /// Represents the interface where the objects will be drawn.
    pub interface: &'a Interface,
    color_system: ColorSystem,
    image_system: ImageSystem,
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

    pub fn draw_obj(&mut self, object: &ObjectWrapper) {
        let camera_transform: transform::Transform =
            self.interface.camera.get_transformation(Vec2::ZERO);

        match object {
            ObjectWrapper::SolidColorObject(object) => self.color_system.draw(
                &mut self.target,
                &self.interface.display,
                camera_transform,
                object,
            ),
            ObjectWrapper::ImageObject(object) => self.image_system.draw(
                &mut self.target,
                &self.interface.display,
                camera_transform,
                object,
            ),
        }
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

    pub fn draw_buffer<T: Iterator<Item = ObjectWrapper>>(&mut self, buffer: T) {
        for object in buffer {
            self.draw_obj(&object);
        }
    }
}
