use crate::{vector2::Vec2, vec2};
use std::ops::Deref;
use super::Rect;

/// A 4x4 matrix of floating point numbers.
///
/// This type is used to represent a 4x4 matrix of floating point numbers.
/// It is used in the `mult_matrix` function to perform matrix multiplication.
pub type Matrix = [[f32; 4]; 4];

/// Multiplies two matrices.
///
/// This function takes two 4x4 matrices and returns their product.
/// The matrices are represented as 2D arrays of floating point numbers.
///
/// # Examples
///
/// ```
/// let x = [[1. , 2., 3., 4.],
///          [5. , 6., 7., 8.], 
///          [9. , 10., 11., 12.], 
///          [13., 14., 15., 16.]]; 
/// 
/// let y = [[17., 18., 19., 20.], 
///          [21., 22., 23., 24.], 
///          [25., 26., 27., 28.], 
///          [29., 30., 31., 32.]]; 
/// let result = mult_matrix(&x, &y); 
/// ```
fn mult_matrix<const A: usize, const B: usize, const C: usize>(
    x: &[[f32; A]; B],
    y: &[[f32; B]; C],
) -> [[f32; A]; C] {
    let mut result = [[0.; A]; C];
    for i in 0..A {
        for j in 0..C {
            result[i][j] = (0..B).map(|k| x[i][k] * y[k][j]).sum();
        }
    }
    result
}

/// A transformation matrix.
///
/// This struct represents a transformation matrix. It is a wrapper around a 4x4 matrix of floating point numbers.
/// It provides methods to apply transformations such as translation and scaling.
pub struct Transform(pub Matrix);

impl Deref for Transform {
    type Target = Matrix;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ])
    }
}

impl Transform {
    /// Applies a transformation to the matrix.
    ///
    /// This method takes another matrix and multiplies it with the current matrix.
    /// The result is a new matrix that represents the combined transformation.
    pub fn apply_trasnformation(mut self, other: &Matrix) -> Self {
        self.0 = mult_matrix(&self.0, other);
        self
    }

    /// Applies a translation transformation.
    ///
    /// This method takes a 2D vector and applies a translation transformation to the matrix.
    /// The translation is applied by adding the vector to the current matrix.
    pub fn translate(self, translation: Vec2) -> Self {
        self.apply_trasnformation(&transform_funcs::get_translation(translation))
    }

    /// Applies a scaling transformation.
    ///
    /// This method takes a 2D vector and applies a scaling transformation to the matrix (where x of the 2D vector is the factor on the x-axis and y on the y-axis).
    /// The scaling is applied by multiplying the vector with the current matrix.
    pub fn scale(self, scale: Vec2) -> Self {
        self.apply_trasnformation(&transform_funcs::get_scale(scale))
    }
}

/// A module containing helper functions for transformations.
///
/// This module provides helper functions for creating transformation matrices.
/// These matrices can be used to apply transformations such as translation and scaling.
pub mod transform_funcs {
    use crate::vector2::Vec2;

    use super::*;

    /// Returns a translation matrix.
    ///
    /// This function takes a 2D vector and returns a 4x4 matrix that represents a translation transformation.
    /// The translation is applied by adding the vector to the current matrix.
    pub fn get_translation(translation: Vec2) -> Matrix {
        [
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [translation.x, translation.y, 0., 1.],
        ]
    }

    /// Returns a scaling matrix.
    ///
    /// This function takes a 2D vector and returns a 4x4 matrix that represents a scaling transformation.
    /// The scaling is applied by multiplying the vector with the current matrix.
    pub fn get_scale(scale: Vec2) -> Matrix {
        [
            [scale.x, 0., 0., 0.],
            [0., scale.y, 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]
    }
}

/// A camera.
///
/// This struct represents a camera. It has a world rectangle and a target rectangle.
/// The world rectangle represents the area of the world that the camera can see.
/// The target rectangle represents the area of the screen where the world will be rendered.
pub struct Camera {
    /// The world rectangle.
    ///
    /// This field represents the area of the world that the camera can see.
    /// It is a rectangle that defines the boundaries of the world in the game world coordinates.
    pub world: Rect,

    /// The target rectangle.
    ///
    /// This field represents the area of the screen where the world will be rendered.
    /// It is a rectangle that defines the boundaries of the screen in the screen coordinates.
    pub target: Rect,
}
impl Camera {
    /// Returns a transformation matrix for the camera.
    ///
    /// This method takes a 2D vector and returns a transformation matrix that represents the camera's transformation.
    /// The transformation is calculated based on the camera's world and target rectangles.
    pub fn transformation(&self) -> Transform {
        let scale = self.scale();
        Transform::default()
            .translate(self.world.center)
            .scale(scale)
            .translate(self.target.center)
    }

    /// The element-wise ratio of target to word size
    pub fn scale(&self) -> Vec2 {
        Vec2 {
            x: self.target.size.x / self.world.size.x,
            y: self.target.size.y / self.world.size.y,
        }
    }
}