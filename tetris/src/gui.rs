//! Where access and manipulation to visual interface components is available, such as camera, transformations, window manipulations, etc.

mod transform;

/// Module for accessing the interface, with wrappers for communicating with the interface
pub mod interface;

use glium::implement_vertex;
pub use transform::*;

/// Import the vec2 and vector2::Vec2 modules.
use crate::{vec2, vector2::Vec2};

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
#[derive(Debug,Copy, Clone)]
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
      self.center.x - self.size.x/2. 
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
      self.center.x + self.size.x/2. 
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
      self.center.y - self.size.y/2. 
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
      self.center.y + self.size.y/2. 
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
  pub fn complete_in(self, other: Rect ) -> bool {
      other.left() <= self.left() && self.right() <= other.right() &&
      other.bottom() <= self.bottom() && self.top() <= other.top()
  }
}

/// A struct representing a vertex.
#[derive(Copy, Clone)]
struct Vertex {
   /// The position of the vertex.
   position: [f32; 2],
   /// The color of the vertex.
   color: [f32; 3],
}


implement_vertex!(Vertex, position, color);

/// A struct representing an object in the interface, with color and format.
#[derive(Debug)]
pub struct Object {
   /// The format of the object.
   pub format: Rect,
   /// The color of the object.
   pub color: [f32; 3],
}

impl Object {
   /// Converts the object to a vertex buffer.
   ///
   /// # Examples
   ///
   /// ```
   /// let object = Object {
   ///    format: Rect {
   ///        size: Vec2 { x: 10.0, y: 20.0 },
   ///        center: Vec2 { x: 5.0, y: 10.0 },
   ///    },
   ///    color: [1.0, 0.0, 0.0],
   /// };
   /// let vertices = object.to_vertex_buffer();
   ///```
   fn to_vertex_buffer(&self) -> Vec<Vertex> {
       let Rect { center, size } = self.format;
       vec![
           Vertex {
               position: (center - size / 2.).into(),
               color: self.color,
           },
           Vertex {
               position: (center + vec2!(size.x, -size.y) / 2.).into(),
               color: self.color,
           },
           Vertex {
               position: (center + size / 2.).into(),
               color: self.color,
           },
           Vertex {
               position: (center + size / 2.).into(),
               color: self.color,
           },
           Vertex {
               position: (center + vec2!(-size.x, size.y) / 2.).into(),
               color: self.color,
           },
           Vertex {
               position: (center - size / 2.).into(),
               color: self.color,
           },
       ]
   }
}