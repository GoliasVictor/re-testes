//! This module contains the `Vector2` struct and related functions.
//!
//! The [Vector2](crate::core::vector2::Vector2) struct is a generic 2D vector that can be used to represent
//! points in a 2D space. It includes methods for creating a new vector and
//! for creating common direction vectors.
//!
//! # Examples
//!
//! ```
//! let a = Vector2::new(1.0, 1.0); 
//! let b = Vector2::new(2.0, -1.0); 
//! let c = a + b * 2; 
//! assert_eq!(a.x, 1.0); 
//! assert_eq!(a.y, 1.0); 
//! assert_eq!(b.x, 2.0); 
//! assert_eq!(b.y, -1.0); 
//! assert_eq!(c.x, 5.0); 
//! assert_eq!(c.y, -1.0); 
//!```
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign, DivAssign};


/// The `Field` trait represents a mathematical field.
///
/// A field is a set of numbers with the following operations defined:
///
/// - Addition ([Add] trait)
/// - Subtraction ([Sub] trait)
/// - Multiplication ([Mul] trait)
/// - Division ([Div] trait)
///
/// The operations are required to be associative, commutative, and distributive over the field.
pub trait Field = Sized
    + Copy
    + Clone
    + Add<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Sub<Output = Self>;

 /// A 2D vector struct.
///
/// This struct is generic over the type `T` which must implement the `Field` trait.
/// It represents a 2D vector with `x` and `y` components of type `T`.
///
/// # Examples
///
/// ```
/// let v = Vector2::new(1.0, 2.0); 
/// assert_eq!(v.x, 1.0); 
/// assert_eq!(v.y, 2.0); 
///```

#[derive(Clone, Copy, Debug)]
pub struct Vector2<T: Field>  {
  /// The x component of the vector.
  pub x: T,
  /// The y component of the vector.
  pub y: T,
}

impl<T: Field> Vector2<T>{
   /// Creates a new `Vector2`.
   ///
   /// # Examples
   ///
   /// ```
   /// let v = Vector2::new(1.0, 2.0); 
   /// assert_eq!(v.x, 1.0); 
   /// assert_eq!(v.y, 2.0); 
   /// ```
   
    #[inline]
    pub fn new<K: Into<T>>( x: K, y: K) -> Self {
        Vector2 {
            x: x.into(), 
            y: y.into()
        }
    }
}

/// This trait [ToVec2] is implemented for different types that can be converted to a `Vector2`.
pub trait ToVec2<T : Field> {
    /// Converts to a [Vector2].
    fn to_vec2(&self) -> Vector2<T>;   
 }

impl<T : Field , K : Field + Into<T>> ToVec2<T> for Vector2<K> {
    fn to_vec2(&self) -> Vector2<T> {
        Vector2 {
            x: self.x.into(),
            y: self.y.into()
        }
    }
} 
impl<T : Field> ToVec2<T> for (T,T) {
    fn to_vec2(&self) -> Vector2<T> {
        Vector2 {
            x: self.0,
            y: self.1
        }
    }
} 
impl<T : Field> ToVec2<T> for [T;2] {
    fn to_vec2(&self) -> Vector2<T> {
        Vector2 {
            x: self[0],
            y: self[1]
        }
    }
} 
/// A type alias for `Vector2<f32>`.
pub type Vec2 = Vector2<f32>;

impl Vec2 {
   /// The up direction vector.
   pub const UP: Vec2 = Vec2 { x: 0.0, y: 1.0 };
   /// The down direction vector.
   pub const DOWN: Vec2 = Vec2 { x: 0.0, y: -1.0 };
   /// The left direction vector.
   pub const LEFT: Vec2 = Vec2 { x: -1.0, y: 0.0 };
   /// The right direction vector.
   pub const RIGHT: Vec2 = Vec2 { x: 1.0, y: 0.0 };
   /// The zero vector.
   pub const ZERO: Vec2 = Vec2 { x: 0., y: 0. };
}
/// # Examples
///
/// ```
/// let vf = vec2!(1.0, 2.0); 
/// assert_eq!(v.x, 1.0); 
/// assert_eq!(v.y, 2.0); 
/// 
/// let vi = vec2!(1, 2); 
/// assert_eq!(v.x, 1); 
/// assert_eq!(v.y, 2); 
/// ```
#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        $crate::vector2::Vector2::new($x, $y)
    };
}

impl<T: Field> From<Vector2<T>> for [T; 2] {
    fn from(value: Vector2<T>) -> Self {
        [value.x, value.y]
    }
}



impl<T: Field> Add for Vector2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Field> AddAssign for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T: Field> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl<T: Field> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<T: Field + Neg<Output = T>> Neg for Vector2<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vector2::<T> {
            x: -self.x,
            y: -self.y,
        }
    }
}
impl<T: Field> Mul<T> for Vector2<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T: Field> Div<T> for Vector2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Field> DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}
