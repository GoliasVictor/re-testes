use std::rc::Rc;

use glium::{glutin::event::VirtualKeyCode, texture::SrgbTexture2d};

use crate::{gui::{interface::{Interface, Canvas}, ObjectWrapper, systems::{ImageObject, TextObject, FontID}, Rect}, include_png, vector2::Vec2};

pub struct HomeScene {
	brick: Rc<SrgbTexture2d>

}

impl HomeScene {
    pub fn new(columns: i16, rows: i16, interface: &Interface) -> HomeScene {
        HomeScene {
			brick: interface.create_texture(include_png!("../assets/brick.png"))
		}
    }
    /// Receives the keypress event
    pub fn key_down(&mut self, key: VirtualKeyCode) {
		
    }

    /// Updates the game state and draws on the table
    pub fn update(&mut self, canvas: &mut Canvas, _delta_t: u128){
		canvas.draw_obj(&ObjectWrapper::ImageObject(ImageObject{
			
			region: Rect {
				center:  vec2!(canvas.interface.camera.world_showed().left()+50. , 0.) ,
				size: vec2!(100., 100.),
			},
			texture: self.brick.clone()
		}));

		canvas.draw_obj(&ObjectWrapper::TextObject(TextObject { text: "HELLO".into(), font: FontID(1) }));
    }

	pub fn get_center_map(&self) -> Vec2 {
		vec2!(0., 0.)
	}
}
