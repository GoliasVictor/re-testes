//! Module containing the specific mechanics of the Tetris game, such as receiving events, etc.
mod level_scene;
mod home_scene;
mod bag;
use glium::glutin::event::VirtualKeyCode;
use home_scene::HomeScene;
use level_scene::LevelScene;

use crate::{gui::{
    interface::{Canvas, Interface},
    Rect, systems::SolidColorObject,
}, vector2::Vec2, core::rgb};
#[derive(PartialEq, Eq)]

pub enum Scene {
    HomeScene,
    LevelScene,
}

/// The state of all game logic
pub struct GameState {
    actual_scene: Scene,
    level_scene: LevelScene,
    home_scene: HomeScene,
}

impl GameState {
    /// Create a new GameState
    pub fn new(interface: &Interface) -> GameState {
        GameState {
            actual_scene: Scene::HomeScene,
            level_scene: LevelScene::new(interface),
            home_scene: HomeScene::new(interface)
        }
    }
    fn update_scene(&mut self, new_scene: Scene){
        if self.actual_scene == new_scene {
            return;
        }
        self.actual_scene = new_scene;        
    }
    /// Returns the region in the world being shown
    pub fn world_region(&self) -> Rect {
        match self.actual_scene {
            Scene::HomeScene => self.home_scene.world_region(),
            Scene::LevelScene => self.level_scene.world_region(),
        }
    }
    /// Receives the keypress event
    pub fn key_down(&mut self, key: VirtualKeyCode) {
        let new_scene = match self.actual_scene {
            Scene::HomeScene => self.home_scene.key_down(key),
            Scene::LevelScene => self.level_scene.key_down(key),
        };
        self.update_scene(new_scene);
        match key {
            VirtualKeyCode::Key1 => self.actual_scene = Scene::HomeScene,
            VirtualKeyCode::Key2 => self.actual_scene = Scene::LevelScene,
            _ => (),
        }
    }

    /// Recives the mouse click event
    pub fn on_click(&mut self, position: Vec2){
        let new_scene = match self.actual_scene {
            Scene::HomeScene => self.home_scene.on_click(position),
            Scene::LevelScene => self.level_scene.on_click(position),
        };
        self.update_scene(new_scene);
    }

    /// Updates the game state and draws on the table
    pub fn update(&mut self, canvas: &mut Canvas, delta_t: u128) {
        let new_scene =match self.actual_scene {
            Scene::HomeScene => self.home_scene.update(canvas, delta_t),
            Scene::LevelScene => self.level_scene.update(canvas, delta_t),
        };
        self.update_scene(new_scene);
    }
}
