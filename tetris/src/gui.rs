//! Where access and manipulation to visual interface components is available, such as camera, transformations, window manipulations, etc.

pub mod interface;
pub mod systems;
mod transform;

pub use transform::*;

/// Import the vec2 and vector2::Vec2 modules.
use crate::vector2::Vec2;

use self::systems::{ImageObject, SolidColorObject, TextObject};

/// Load the bytes in compilation time, and in runtime convert to rgba
#[macro_export]
macro_rules! include_png {
    ($file:expr $(,)?) => {{
        let image = image::load(
            std::io::Cursor::new(&include_bytes!($file)),
            image::ImageFormat::Png,
        )
        .unwrap()
        .to_rgba8();

        let dimensions = image.dimensions();
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions)
    }};
}
/// A struct representing a rectangle.
///
/// # Examples
///
/// ```
/// # fn main(){
/// let rect = Rect {
///    size: Vec2 { x: 10.0, y: 20.0 },
///    center: Vec2 { x: 5.0, y: 10.0 },
/// };
/// assert_eq!(rect.left(), 0.0);
/// assert_eq!(rect.right(), 10.0);
/// assert_eq!(rect.bottom(), 0.0);
/// assert_eq!(rect.top(), 20.0);
/// # }
///```
#[derive(Debug, Copy, Clone)]
pub struct Rect {
    /// The size of the rectangle.
    pub size: Vec2,
    /// The center of the rectangle.
    pub center: Vec2,
}

impl Rect {
    /// Returns the left edge of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main(){
    /// let rect = Rect { center: vec2!(0., 0.), size: vec2!(10., 10.) };
    /// assert_eq!(rect.left(), -5.0);
    /// # }
    ///```
    pub fn left(&self) -> f32 {
        self.center.x - self.size.x / 2.
    }

    /// Returns the right edge of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main(){
    /// let rect = Rect { center: vec2!(0., 0.), size: vec2!(10., 10.) };
    /// assert_eq!(rect.right(), 5.0);
    /// # }
    ///```
    pub fn right(&self) -> f32 {
        self.center.x + self.size.x / 2.
    }

    /// Returns the bottom edge of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main(){
    /// let rect = Rect { center: vec2!(0., 0.), size: vec2!(10., 10.) };
    /// assert_eq!(rect.bottom(), -5.0);
    /// # }
    ///```
    pub fn bottom(&self) -> f32 {
        self.center.y - self.size.y / 2.
    }

    /// Returns the top edge of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main(){
    /// let rect = Rect { center: vec2!(0., 0.), size: vec2!(10., 10.) };
    /// assert_eq!(rect.top(), 5.0);
    /// # }
    ///```
    pub fn top(&self) -> f32 {
        self.center.y + self.size.y / 2.
    }

    /// Checks if the rectangle is completely inside another rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main(){
    /// let rect1 = Rect { center: vec2!(0., 0.), size: vec2!(10., 10.) };
    /// let rect2 = Rect { center: vec2!(0., 0.), size: vec2!(20., 20.) };
    /// assert!(rect1.complete_in(rect2));
    /// # }
    ///```
    pub fn complete_in(self, other: Rect) -> bool {
        other.left() <= self.left()
            && self.right() <= other.right()
            && other.bottom() <= self.bottom()
            && self.top() <= other.top()
    }
}

/// A enum wrapping differents types of objects to draw in screen.
pub enum ObjectWrapper {
    /// Wrapper for a object of  SolidColor
    SolidColorObject(SolidColorObject),
    /// Wrapper for a objec of a image
    ImageObject(ImageObject),
    TextObject(TextObject)
}
