/// This module contains the `Vector2` struct and related functions.
///
/// The [Vector2](crate::core::vector2::Vector2) struct is a generic 2D vector that can be used to represent
/// points in a 2D space. It includes methods for creating a new vector and
/// for creating common direction vectors.
///
/// # Examples
///
/// ```
/// let a = Vector2::new(1.0, 1.0); 
/// let b = Vector2::new(2.0, -1.0); 
/// let c = a + 2 * b; 
/// assert_eq!(a.x, 1.0); 
/// assert_eq!(a.y, 1.0); 
/// assert_eq!(b.x, 2.0); 
/// assert_eq!(b.y, -1.0); 
/// assert_eq!(c.x, 5.0); 
/// assert_eq!(c.y, -1.0); 
///```
#[macro_use]
pub mod vector2;
