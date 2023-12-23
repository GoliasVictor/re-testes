mod level_scene;
mod home_scene;
use glium::glutin::event::VirtualKeyCode;
use home_scene::HomeScene;
use level_scene::LevelScene;

use crate::{gui::interface::{Interface, Canvas}, vector2::Vec2};
enum Scenes {
    HomeScene,
    LevelScene
}

/// The size of a tetramino in the map 
pub const SIZE: f32 = 1.;
pub struct GameState {
    actual_scene: Scenes,
    level_scene : LevelScene,
    home_scene : HomeScene
}

impl GameState {
    pub fn new(columns: i16, rows: i16, interface: &Interface) -> GameState {
        GameState {
            actual_scene: Scenes::HomeScene,
            level_scene: LevelScene::new(columns, rows, interface),
            home_scene: HomeScene::new(columns, rows, interface)
        }
    }
    pub fn get_center_map(&self) -> Vec2 {
        match self.actual_scene {
            Scenes::HomeScene => self.home_scene.get_center_map(),
            Scenes::LevelScene => self.level_scene.get_center_map()
        }
    }
    /// Receives the keypress event
    pub fn key_down(&mut self, key: VirtualKeyCode) {
        match self.actual_scene {
            Scenes::HomeScene => self.home_scene.key_down(key),
            Scenes::LevelScene => self.level_scene.key_down(key)
        }
    }

    /// Updates the game state and draws on the table
    pub fn update(&mut self, canvas: &mut Canvas, delta_t: u128){
        match self.actual_scene {
            Scenes::HomeScene => self.home_scene.update(canvas, delta_t),
            Scenes::LevelScene => self.level_scene.update(canvas, delta_t)
        }
    } 
    
}

