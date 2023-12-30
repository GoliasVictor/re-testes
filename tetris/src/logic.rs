//! Module containing the specific mechanics of the Tetris game, such as receiving events, etc.
mod level_scene;
mod home_scene;
mod bag;
use glium::glutin::event::VirtualKeyCode;
use home_scene::HomeScene;
use level_scene::LevelScene;

use crate::gui::{
    interface::{Canvas, Interface},
    Rect,
};
enum Scenes {
    HomeScene,
    LevelScene,
}

/// The state of all game logic
pub struct GameState {
    actual_scene: Scenes,
    level_scene: LevelScene,
    home_scene: HomeScene,
}

impl GameState {
    /// Create a new GameState
    pub fn new(columns: i16, rows: i16, interface: &Interface) -> GameState {
        GameState {
            actual_scene: Scenes::HomeScene,
            level_scene: LevelScene::new(columns, rows, interface),
            home_scene: HomeScene::new(interface),
        }
    }
    /// Returns the region in the world being shown
    pub fn world_region(&self) -> Rect {
        match self.actual_scene {
            Scenes::HomeScene => self.home_scene.world_region(),
            Scenes::LevelScene => self.level_scene.world_region(),
        }
    }
    /// Receives the keypress event
    pub fn key_down(&mut self, key: VirtualKeyCode) {
        match self.actual_scene {
            Scenes::HomeScene => self.home_scene.key_down(key),
            Scenes::LevelScene => self.level_scene.key_down(key),
        }
        match key {
            VirtualKeyCode::Key1 => self.actual_scene = Scenes::HomeScene,
            VirtualKeyCode::Key2 => self.actual_scene = Scenes::LevelScene,
            _ => (),
        }
    }

    /// Updates the game state and draws on the table
    pub fn update(&mut self, canvas: &mut Canvas, delta_t: u128) {
        match self.actual_scene {
            Scenes::HomeScene => self.home_scene.update(canvas, delta_t),
            Scenes::LevelScene => self.level_scene.update(canvas, delta_t),
        }
    }
}
