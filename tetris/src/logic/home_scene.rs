use std::rc::Rc;

use glium::{glutin::event::VirtualKeyCode, texture::SrgbTexture2d};

use crate::{
    gui::{
        interface::{Canvas, Interface},
        systems::{TextObject, SolidColorObject},
        Rect,
    },
    include_png, core::rgb::Rgb, vector2::Vec2,
};

use super::Scene;

pub struct HomeScene {
    brick: Rc<SrgbTexture2d>,
}

impl HomeScene {
    const BUTTON_REGION: Rect = Rect {
            center:  Vec2 {x : 0., y: 0.} ,
            size: Vec2 { x : 35.+5., y: 15.},
        };

    pub fn new(interface: &Interface) -> HomeScene {
        HomeScene {
            brick: interface.create_texture(include_png!("../assets/brick.png")),
        }
    }
    /// Receives the keypress event
    pub fn key_down(&mut self, _key: VirtualKeyCode) -> Scene {
        Scene::HomeScene
    }
    pub fn on_click(&mut self, position : Vec2) -> Scene {
        if Self::BUTTON_REGION.left() < position.x && position.x < Self::BUTTON_REGION.right()
        && Self::BUTTON_REGION.bottom() < position.y && position.y < Self::BUTTON_REGION.top(){
            return Scene::LevelScene;
        }
        Scene::HomeScene
    }
    /// Updates the game state and draws on the table
    pub fn update(&mut self, canvas: &mut Canvas, _delta_t: u128) -> Scene {

        canvas.draw(TextObject {
            text: "Tetris".into(),
            position: vec2!(-30., 80.),
            color: Rgb::WHITE,
            font_size: 20.,
        });

        canvas.draw( SolidColorObject{
            region: Self::BUTTON_REGION,
            color: Rgb::WHITE
        });
        canvas.draw(TextObject {
            text: "Começar".into(),
            position: vec2!(-17.5, 5.),
            color: Rgb::BLACK,
            font_size: 10.,
        });

        canvas.draw(SolidColorObject{
        	region: Rect {
        		center:  vec2!(0., 0.) ,
        		size: vec2!(35.+5., 15.),
        	},
			color: Rgb::WHITE
		});
        canvas.draw(TextObject {
            text: "Começar".into(),
            position: vec2!(-17.5, 5.),
            color: Rgb::BLACK,
            font_size: 10.,
        });
        Scene::HomeScene
    }

    pub fn world_region(&self) -> Rect {
        Rect {
            center: vec2!(0., 0.), 
            size: vec2!(100., 100.)
        }
    }
}
