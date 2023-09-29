use glium::glutin::event::VirtualKeyCode;
use rand::{rngs::ThreadRng, Rng};
use crate::gui::{ Object, Rect, interface::Canvas};

#[derive(Clone, Debug)]
/// Used to create the default Tetraminos
struct TetraminoTemplate {
    /// Binary number for the blocks, first four represent top row, last four represent bottom 
    blocks: u8, 
    color: (u8, u8, u8),
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
    block_positions: [Option<(f64, f64)>; 4],
    color:  (u8, u8, u8),
}

impl Tetramino {
    fn new(template: TetraminoTemplate) -> Tetramino {
        let mut block_positions: [Option<(f64, f64)>; 4] = [Some((0.0,0.0)); 4];
        let mut x: u8 = 0;
        let mut y: u8 = 0;
        let mut i: u8 = 0;
        while (x + (4*y)) < 8 && i < 4 {
            if template.blocks & (1 << (x + (4*y))) != 0 {
                block_positions[i as usize] = Some(((x as f64) + 1.0/4.0, (y as f64) + 1.0/4.0));
                i += 1;
            }

            x += 1;

            if x >= 4 {
                x = 0;
                y += 1;
            }
        }
        
        return Tetramino {
            color: template.color,
            block_positions: block_positions,
        };
    }
    /// Get tetramino center relative to its blocks
    fn get_center(&mut self) -> (f64, f64) { 
        let mut center: (f64, f64) = (0.0, 0.0);
        let mut block_count: u8 = 0;

        for i in 0..4 {
            match self.block_positions[i] {
                None => continue,
                Some(block) => {
                    block_count += 1;
                    center.0 += block.0;
                    center.1 += block.1;
                },
            };
        }

        center.0 /= block_count as f64;
        center.1 /= block_count as f64;

        center
    }

    fn rotate(&mut self) {
        let center = self.get_center();
        for i in 0..4 {
            match self.block_positions[i] {
                None => continue,
                Some(block) => {
                    let mut relative_position: (f64, f64) = center;
                    relative_position.0 -= block.0;
                    relative_position.1 -= block.1;
            
                    let mut x_multiplier: i8 = 1;
                    let mut y_multiplier: i8 = 1;

                    if relative_position.0 <= 0.0 { y_multiplier = -1 }
                    if relative_position.1 >= 0.0 { x_multiplier = -1 }

                    let mut position: (f64, f64) = (0.0, 0.0);
                    position.0 = center.0 + (relative_position.1.abs() * x_multiplier as f64);
                    position.1 = center.1 + (relative_position.0.abs() * y_multiplier as f64);
                    self.block_positions[i] = Some(position);
                },
            }
        }
    }
}

#[derive(Debug)]
pub struct Player {
    tetramino: Tetramino,
    position: (u8, u8),
}
fn to_object(position: (u8, u8), color: (u8, u8, u8)) -> Object {
    const SIZE: f32 = 5.;
    let mut f_color: [f32; 3] = [0.0; 3];
    f_color[0] = color.0 as f32 / 255.0;
    f_color[1] = color.1 as f32 / 255.0;
    f_color[2] = color.2 as f32 / 255.0;


    return Object {
        format: Rect {
            center: vec2!(
                (position.0 as f32 + 0.5) * SIZE,
                (position.1 as f32 + 0.5) * SIZE
            ),
            size: vec2!(SIZE, SIZE),
        },
        color: f_color,
    };
}

impl Player {
    fn to_object_buffer(&self) -> Vec<Object> {
        let mut buffer = vec![];
        for i in 0..4 {
            match self.tetramino.block_positions[i] {
                None => continue,
                Some(block) => {
                    let x: f64 = self.position.0 as f64 + block.0;
                    let y: f64 = self.position.1 as f64 + block.1;
                    buffer.push(to_object((x as u8, y as u8), self.tetramino.color));
                },
            }
        }
        buffer
    }
}
#[derive(Debug)]
pub struct GameState {
    player: Player,
    rng: ThreadRng,
    columns: u8,
    rows: u8,
	time: u128,
    max_time: u128,
}

impl GameState {
    fn next_player(&mut self) -> Player {
		let tetramino = Tetramino::new(TETRAMINO_TEMPLATES[self.rng.gen_range(0..7)].clone());
        Player {
			position: (((self.columns as f32 / 2.).ceil() as u8), self.rows),
			tetramino 
		}
    }
    pub fn new(columns: u8, rows: u8) -> GameState {
        let mut rng = rand::thread_rng();

        let tetramino = Tetramino::new(TETRAMINO_TEMPLATES[rng.gen_range(0..7)].clone());
        let player = Player {
			position: (((columns as f32 / 2.).ceil() as u8), rows),
			tetramino 
		};

        GameState {
			time: 0,
            player: player,
			rng: rng,
            columns: columns,
            rows: rows,
            max_time: 1000000,
        }
	}
	pub fn key_press(&mut self, key : VirtualKeyCode) {
		match key {
    		VirtualKeyCode::R => {
			    self.player.tetramino.rotate();
		    },
			VirtualKeyCode::N => self.player = self.next_player(),
    		_ => (),
		}
	}
    pub fn update(&mut self, canvas: &mut Canvas, delta_t : u128) {
        canvas.draw_buffer(self.player.to_object_buffer().into_iter());
		self.time += delta_t;
		if self.time >= self.max_time {
			if self.player.position.1 > 0 {

				self.player.position.1 -= 1;
			}
			self.time -= self.max_time;
		}  
    }
}
