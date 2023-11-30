use glium::glutin::event::VirtualKeyCode;
use rand::{rngs::ThreadRng, Rng};
use crate::{gui::{ Object, Rect, interface::Canvas}, vector2::{Vector2, Vec2, ToVec2}};

#[derive(Clone, Debug)]
/// Used to create the default Tetraminos
struct TetraminoTemplate {
    /// Binary number for the blocks, first four represent top row, last four represent bottom 
    blocks: i16, 
    color: (i16, i16, i16),
}

const TETRAMINO_TEMPLATES: [TetraminoTemplate; 7] = [
    TetraminoTemplate{blocks: 0b11001100, color: (241, 196, 15)}, // Square
    TetraminoTemplate{blocks: 0b11100100, color: (142, 68, 173)}, // T
    TetraminoTemplate{blocks: 0b00101110, color: (230, 126, 34)}, // L
    TetraminoTemplate{blocks: 0b10001110, color: (41, 128, 185)}, // Reverse L
    TetraminoTemplate{blocks: 0b11110000 , color: (93, 173, 226)}, // Straight
    TetraminoTemplate{blocks: 0b11000110, color: (231, 76, 60)}, // Z
    TetraminoTemplate{blocks: 0b01101100, color: (46, 204, 113)} // S
];


#[derive(Clone,Debug)]
pub struct Tetramino {
    block_positions: [Option<Vec2>; 4],
    color:  (i16, i16, i16),
}

impl Tetramino {
    fn new(template: TetraminoTemplate) -> Tetramino {
        let mut block_positions: [Option<Vec2>; 4] = [Some(vec2!(0.0,0.0)); 4];
        let mut x: i16 = 0;
        let mut y: i16 = 0;
        let mut i: i16 = 0;
        while (x + (4*y)) < 8 && i < 4 {
            if template.blocks & (1 << (x + (4*y))) != 0 {
                block_positions[i as usize] = Some(vec2!((x as f32) + 1.0/4.0, (y as f32) + 1.0/4.0));
                i += 1;
            }

            x += 1;

            if x >= 4 {
                x = 0;
                y += 1;
            }
        }
        
        Tetramino {
            color: template.color,
            block_positions,
        }
    }
    /// Get tetramino center relative to its blocks
    fn get_center(&mut self) -> Vec2 { 
        let mut center = Vec2::ZERO;
        let mut block_count: i16 = 0;
        self.block_positions.iter().flatten().for_each(|block| {
            block_count += 1;
            center.x += block.x;
            center.y += block.y;
        });
        center /= block_count as f32;
        center
    }

    fn rotate(&mut self) {
        let center = self.get_center();
        for i in 0..4 {
            if let Some(block) = self.block_positions[i] {
                let relative_position = center - block;
            
                let mut x_multiplier = 1.;
                let mut y_multiplier = 1.;

                if relative_position.x <= 0.0 { y_multiplier = -1. }
                if relative_position.y >= 0.0 { x_multiplier = -1. }

                let mut position = Vec2::ZERO;
                position.x = center.x + (relative_position.y.abs() * x_multiplier as f32);
                position.y = center.y + (relative_position.x.abs() * y_multiplier as f32);
                self.block_positions[i] = Some(position);
            }
        }
    }
}

#[derive(Debug)]
pub struct Player {
    tetramino: Tetramino,
    position: Vector2<i16>,
}
pub const SIZE: f32 = 5.;
fn to_object(position: Vector2<i16>, color: (i16, i16, i16)) -> Object {
    let mut f_color: [f32; 3] = [0.0; 3];
    f_color[0] = color.0 as f32 / 255.0;
    f_color[1] = color.1 as f32 / 255.0;
    f_color[2] = color.2 as f32 / 255.0;
    Object {
        format: Rect {
            center: (position.to_vec2() + vec2!(0.5, 0.5))*SIZE,
            size: vec2!(SIZE, SIZE),
        },
        color: f_color,
    }
}

impl Player {
    fn to_object_buffer(&self) -> Vec<Object> {
        self.tetramino.block_positions.into_iter().flatten().map(|block| {
            let pos = self.position.to_vec2() + block;
            to_object(vec2!(pos.x as i16, pos.y as i16), self.tetramino.color)
        }).collect()
    }
    pub fn translate_x(&mut self, delta_x : i16){
        self.position.x += delta_x;
    }

}
#[derive(Debug)]
pub struct GameState {
    player: Player,
    rng: ThreadRng,
    pub columns: i16,
    pub rows: i16,
	time: u128,
    max_time: u128,
}

impl GameState {
    fn next_player(&mut self) -> Player {
		let tetramino = Tetramino::new(TETRAMINO_TEMPLATES[self.rng.gen_range(0..7)].clone());
        Player {
			position: vec2!(((self.columns as f32 / 2.).ceil() as i16) - 2, self.rows -2 ),
			tetramino 
		}
    }
    pub fn new(columns: i16, rows: i16) -> GameState {
        let mut rng = rand::thread_rng();

        let tetramino = Tetramino::new(TETRAMINO_TEMPLATES[rng.gen_range(0..7)].clone());
        let player = Player {
			position: vec2!((columns as f32 / 2.).ceil() as i16 - 2, rows - 2 ),
			tetramino 
		};

        GameState {
			time: 0,
            player,
			rng,
            columns,
            rows,
            max_time: 1000000,
        }
	}
	pub fn key_down(&mut self, key : VirtualKeyCode) {
		match key {
    		VirtualKeyCode::Up => {
			    self.player.tetramino.rotate();
		    },
    		VirtualKeyCode::W => {
				self.player.position.y += 1;
		    },
            VirtualKeyCode::Right => {
                self.player.translate_x(1);
		    },
            VirtualKeyCode::Left => {
                self.player.translate_x(-1);
		    },
			VirtualKeyCode::N => self.player = self.next_player(),
    		_ => (),
		}
	}
    pub fn update(&mut self, canvas: &mut Canvas, delta_t : u128) {


        for i in 0..self.columns {
            for j in 0..self.rows {
                let mut obj = to_object(vec2!(i,j), (64,64,64));
                obj.format.size =   obj.format.size * 0.9;
                canvas.draw_obj(&obj);
            }
        }

        canvas.draw_buffer(self.player.to_object_buffer().into_iter());
        canvas.draw_obj(&Object{
            color: [1.,1.,1.],
            format: Rect {
                center: self.player.position.to_vec2() * SIZE,
                size: vec2!(1., 1.)
            }
        });
        
        canvas.draw_obj(&Object{
            color: [1.,1.,1.],
            format: Rect {
                center: self.player.position.to_vec2() * SIZE + self.player.tetramino.get_center() * SIZE,
                size: vec2!(1., 1.)
            }
        });

		self.time += delta_t;
		if self.time >= self.max_time {
			if self.player.position.y > 0 {
				self.player.position.y -= 1;
			}
			self.time -= self.max_time;
		}  
    }
}
