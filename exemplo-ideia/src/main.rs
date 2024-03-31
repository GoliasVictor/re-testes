mod core {
    use std::ops;
    pub trait CanvasOutput {
        fn size(&self) -> Vec2;
        fn draw_square(&mut self);
        fn draw_circle(&mut self);
        fn draw_line(&mut self);
    }

    pub trait Inputer<T> {
        fn pop_input(&mut self) -> Option<T>;
    }

    pub enum Direction {
        Left,
        Right,
        Top,
        Bottom,
    }

    macro_rules! vec2 {
        ($x:expr,$y:expr) => {
            Vec2 { x: $x, y: $y }
        };
    }

    #[derive(Clone, Copy)]
    pub struct Vec2 {
        pub x: f32,
        pub y: f32,
    }
    pub mod input {
        pub enum MouseEvent {
            RightClick,
            LeftClick,
        }
        pub enum KeyboardInput {
            KeyW,
            KeyA,
            KeyS,
            KeyD,
            ArrowLeft,
            ArrowUp,
            ArrowRight,
            ArrowDown,
            Esc,
        }
        pub enum GeneralInput {
            Mouse(MouseEvent),
            Keyboard(KeyboardInput),
        }
    }
    impl Vec2 {
        pub fn new(x: f32, y: f32) -> Vec2 {
            Vec2 { x, y }
        }
        pub const LEFT: Vec2 = vec2!(-1., 0.);
        pub const RIGHT: Vec2 = vec2!(1., 0.);
        pub const UP: Vec2 = vec2!(0., 1.);
        pub const DOWN: Vec2 = vec2!(0., -1.);
        pub const ZERO: Vec2 = vec2!(0., 0.);
    }
    impl ops::Add<Vec2> for Vec2 {
        type Output = Vec2;
        fn add(self, rhs: Vec2) -> Self::Output {
            return Vec2::new(self.x + rhs.x, self.y + rhs.y);
        }
    }
    impl ops::Sub<Vec2> for Vec2 {
        type Output = Vec2;
        fn sub(self, rhs: Vec2) -> Self::Output {
            return Vec2::new(self.x - rhs.x, self.y - rhs.y);
        }
    }
    impl ops::Mul<f32> for Vec2 {
        type Output = Vec2;
        fn mul(self, rhs: f32) -> Self::Output {
            return Vec2::new(self.x * rhs, self.y * rhs);
        }
    }
    impl ops::Div<f32> for Vec2 {
        type Output = Vec2;
        fn div(self, rhs: f32) -> Self::Output {
            return Vec2::new(self.x / rhs, self.y / rhs);
        }
    }
}

mod Pong {
    use crate::core::{CanvasOutput, Inputer, Vec2};

    #[derive(Clone)]
    struct Wall {
        size: f32,
        position: f32,
        speed: f32,
    }

    use crate::core::input::{GeneralInput, KeyboardInput};
    const LEFT_WALL_ID: usize = 0;
    const RIGHT_WALL_ID: usize = 1;
    struct Game<C: CanvasOutput, I: Inputer<GeneralInput>> {
        canvas_output: C,
        inputer: I,
        walls: [Wall; 2],
        ball_pos: Vec2,
    }
    impl<C: CanvasOutput, I: Inputer<GeneralInput>> Game<C, I> {
        fn update_wall(&mut self, id: usize, dir: f32) {
            let new_pos = self.walls[id].position + self.walls[id].speed * dir;
            self.walls[id].position = new_pos.clamp(0., self.canvas_output.size().y);
        }
        fn run(mut self) {
            loop {
                match self.inputer.pop_input() {
                    Some(GeneralInput::Keyboard(key)) => {
                        match key {
                            KeyboardInput::KeyW => {
                                self.update_wall(LEFT_WALL_ID, 1.)
                            }
                            KeyboardInput::KeyS => {
                                self.update_wall(LEFT_WALL_ID, -1.)
                            }
                            KeyboardInput::ArrowUp => {
                                self.update_wall(RIGHT_WALL_ID, 1.)
                            }
                            KeyboardInput::ArrowDown => {
                                self.update_wall(RIGHT_WALL_ID, -1.)
                            }
                            KeyboardInput::Esc => break,
                            _ => { }
                        };
                    }
                    _ => {}
                }
            }


        }
    }
}
use crate::core::*;

fn main() {
    
    println!("Hello, world!");
}
