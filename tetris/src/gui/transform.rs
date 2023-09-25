use crate::vector2::Vec2;


use std::ops::Deref;



type Matrix = [[f32; 4]; 4];


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
    pub fn apply_trasnformation(mut self, other: &Matrix) -> Self {
        self.0 = mult_matrix(&self.0, other);
        self
    }
    pub fn translate(self, translation: Vec2) -> Self {
        self.apply_trasnformation(&transform_funcs::get_translation(translation))
    }

    pub fn scale(self, scale: Vec2) -> Self {
        self.apply_trasnformation(&transform_funcs::get_scale(scale))
    }
}

pub mod transform_funcs {
    use crate::vector2::Vec2;

    use super::*;

    pub fn get_translation(translation: Vec2) -> Matrix {
        [
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [translation.x, translation.y, 0., 1.],
        ]
    }

    pub fn get_scale(scale: Vec2) -> Matrix {
        [
            [scale.x, 0., 0., 0.],
            [0., scale.y, 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]
    }
}
