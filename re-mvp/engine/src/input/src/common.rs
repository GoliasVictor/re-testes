pub mod common;
pub mod keyboard;
pub mod mouse;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        keyboard::KeyCode,
        mouse::MouseButton,
    }
}
