//! Systems to draw objects in the scree
//!
//! The systems are the set of struct, functions, etc, to draw a specific type of element in the screen

pub mod color_system;
pub mod image_system;
pub mod text_system;
pub use color_system::*;
use glium::Display;
pub use image_system::*;
pub use text_system::*;

use crate::core::rgb::Rgb;

/// A enum wrapping differents types of objects to draw in screen.
pub enum ObjectWrapper {
    /// Wrapper for a object of  SolidColor
    SolidColorObject(SolidColorObject),
    /// Wrapper for a object of a image
    ImageObject(ImageObject),
    /// Wrapper for a object of a text
    TextObject(TextObject),
}

macro_rules! wrap {
    ($name:ident) => {
        impl From<$name> for ObjectWrapper {
            fn from(val: $name) -> Self {
                ObjectWrapper::$name(val)
            }
        }
    };
}

wrap! {SolidColorObject}
wrap! {ImageObject}
wrap! {TextObject}
/// Systems for drawing elements on the screen
pub struct Systems {
	/// System do draw [SolidColorObject] in screen
    pub color_system: ColorSystem,
	/// System do draw [ImageObject] in screen
    pub image_system: ImageSystem,
	/// System do draw [TextObject] in screen
    pub text_system: TextSystem<'static>,
}

impl Systems {
	/// Load each `System` and initialize `Systems`
    pub fn new(display: &Display) -> Systems {
        let color_system = ColorSystem::new(display);
        let image_system = ImageSystem::new(display);
        let text_system = TextSystem::new(display).unwrap();
        Systems {
            color_system,
            image_system,
            text_system,
        }
    }
}

fn rgb_to_arr(rgb: Rgb) -> [f32; 4] {
    [
        rgb.r as f32 / 255.,
        rgb.g as f32 / 255.,
        rgb.b as f32 / 255.,
        1.,
    ]
}
