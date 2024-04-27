use crate::{ButtonInput, ButtonState};

pub enum MouseButton {
    /// The left mouse button.
    Left,
    /// The right mouse button.
    Right,
    /// The middle mouse button.
    Middle,
    /// The back mouse button.
    Back,
    /// The forward mouse button.
    Forward,
    /// Another mouse button with the associated number.
    Other(u16),
}

pub struct PointerMove {
    /// Difference of position since mouse last position
    pub delta: Vec2,
}

pub struct MouseScrollUnit {
    /// Delta of lines or rows to scroll
    Line,
    /// Amount of pixels to scroll
    Pixel,
}

pub struct MouseWheel {
    /// The mouse scroll unit
    pub unit: MouseSrollUnit,
    /// Horizontal scroll movement
    pub x: f32,
    /// Vertical scroll movement
    pub y: f32,
}

pub struct MouseButtonInput {
    /// Mouse button assigned to the event
    pub button: MouseButton,
    /// Press state from mouse button
    pub state: ButtonState,
}
