//! Systems to draw objects in the scree
//! 
//! The systems are the set of struct, functions, etc, to draw a specific type of element in the screen 

pub mod color_system;
pub mod image_system;
pub use color_system::*;
pub use image_system::*;