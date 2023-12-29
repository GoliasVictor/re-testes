use std::rc::Rc;

use glium::{glutin::event::VirtualKeyCode, texture::SrgbTexture2d};

use crate::{
    gui::{
        interface::{Canvas, Interface},
        systems::{TextObject, SolidColorObject, ImageObject},
        ObjectWrapper, Rect,
    },
    include_png,
    vector2::Vec2,
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
    pub fn key_down(&mut self, key: VirtualKeyCode) {}

    /// Updates the game state and draws on the table
    pub fn update(&mut self, canvas: &mut Canvas, _delta_t: u128) {
		canvas.draw_obj(&ObjectWrapper::ImageObject(ImageObject{
			region: Rect {
				center:  vec2!(0., 0.) ,
				size: vec2!(10., 10.),
			},
			texture: self.brick.clone()
		}));
		canvas.draw_obj(&ObjectWrapper::SolidColorObject(SolidColorObject{
        	format: Rect {
        		center:  vec2!(30., -10.) ,
        		size: vec2!(60., 20.),
        	},
			color: [1., 1., 1.]
		}));
        canvas.draw_obj(&ObjectWrapper::TextObject(TextObject {
            text: "Tetris".into(),
            position: vec2!(0., 0.),
            color: [0., 0., 1., 1.],
            font_size: 20.
        }));
    }

    pub fn get_center_map(&self) -> Vec2 {
        vec2!(0., 0.)
    }
}
