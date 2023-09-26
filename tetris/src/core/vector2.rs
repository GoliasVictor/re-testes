use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub const UP: Vec2 = Vec2 { x: 0.0, y: 1.0 };
pub const DOWN: Vec2 = Vec2 { x: 0.0, y: -1.0 };
pub const LEFT: Vec2 = Vec2 { x: -1.0, y: 0.0 };
pub const RIGHT: Vec2 = Vec2 { x: 1.0, y: 0.0 };
pub const ZERO: Vec2 = Vec2 { x: 0., y: 0. };

#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        crate::vector2::Vec2 { x: $x, y: $y }
    };
}

impl From<Vec2> for [f32; 2] {
    fn from(value: Vec2) -> Self {
        [value.x, value.y]
    }
}
impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}
impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}


impl<T: Into<f32> + Copy> Div<T> for Vec2 {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs.into(),
            y: self.y / rhs.into(),
        }
    }
}
