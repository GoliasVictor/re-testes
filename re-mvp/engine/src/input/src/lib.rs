use std::collections;

pub mod common;
pub mod keyboard;
pub mod mouse;

/// Press state of a button
pub enum ButtonState {
    /// Pressed state
    Pressed,
    /// Not pressed state
    Released
}

impl ButtonState {
    fn is_pressed(&self) -> bool {
        matches!(self, ButtonState::Pressed);
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct ButtonInput<T> {
    /// Collection of all buttons being pressed
    pressed: HashSet<T>,
    /// Collection of all buttons pressed during one frame
    frame_pressed: HashSet<T>,
    /// Collection of all buttons released during one frame
    frame_released: HashSet<T>
}

use keyboard::{Key, KeyCode, KeyboardInput};
use mouse::{MouseButton, MouseButtonInput, PointerMove, MouseScrollUnit, MouseWheel};
