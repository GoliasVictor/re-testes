use std::rc::Rc;

use glium::{glutin::event::VirtualKeyCode, texture::SrgbTexture2d};

use crate::{
    gui::{
        interface::{Canvas, Interface},
        systems::{TextObject, SolidColorObject, ImageObject},
        Rect,
    },
    include_png, core::rgb::Rgb,
};

pub struct HomeScene {
    brick: Rc<SrgbTexture2d>,
}

impl HomeScene {
    pub fn new(interface: &Interface) -> HomeScene {
        HomeScene {
            brick: interface.create_texture(include_png!("../assets/brick.png")),
        }
    }
    /// Receives the keypress event
    pub fn key_down(&mut self, _key: VirtualKeyCode) {}

    /// Updates the game state and draws on the table
    pub fn update(&mut self, canvas: &mut Canvas, _delta_t: u128) {

        canvas.draw(TextObject {
            text: "Tetris".into(),
            position: vec2!(-30., 80.),
            color: Rgb::WHITE,
            font_size: 20.,
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
    }

    pub fn world_region(&self) -> Rect {
        Rect {
            center: vec2!(0., 0.), 
            size: vec2!(100., 100.)
        }
    }
}
