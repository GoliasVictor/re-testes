use glium::glutin::event::VirtualKeyCode;
use rand::{rngs::ThreadRng, Rng};

use crate::gui::{Canvas, Object, Rect};

#[derive(Clone,Debug)]
pub struct Tetramino {
    blocks: [[bool; 4]; 2],
}

impl Tetramino {
    fn new(blocks: [[u8; 4]; 2]) -> Tetramino {
        return Tetramino {
            blocks: blocks.map(|line| line.map(|v| v != 0)),
        };
    }
}
#[derive(Debug)]
pub struct Player {
    tetramino: Tetramino,
    rotation: [[i8; 2]; 2],
    position: (u8, u8),
}
fn to_object(position: (u8, u8)) -> Object {
    const SIZE: f32 = 5.;
    return Object {
        format: Rect {
            center: vec2!(
                (position.0 as f32 + 0.5) * SIZE,
                (position.1 as f32 + 0.5) * SIZE
            ),
            size: vec2!(SIZE, SIZE),
        },
        color: [1., 0., 0.],
    };
}

impl Player {
    fn to_object_buffer(&self) -> Vec<Object> {
        let mut buffer = vec![];
        for i in 0..2 {
            for j in 0..4 {
                if self.tetramino.blocks[i as usize][j as usize] {
                    let ni = i * self.rotation[0][0] + j * self.rotation[0][1];
                    let nj = i * self.rotation[1][0] + j * self.rotation[1][1];
                    let x = self.position.0 as i8 + ni;
                    let y = self.position.1 as i8 + nj;
                    buffer.push(to_object((x as u8, y as u8)));
                }
            }
        }
        buffer
    }
}
#[derive(Debug)]
pub struct GameState {
    tetraminos: [Tetramino; 7],
    player: Player,
    rng: ThreadRng,
    columns: u8,
    rows: u8,
	time: u64
}

impl GameState {
    fn next_player(&mut self) -> Player {
        let tetramino = self.tetraminos[self.rng.gen_range(0..7)].clone();
		Player {
			position: (((self.columns as f32 / 2.).ceil() as u8), self.rows),
			rotation: [[1,0], [0,1]],
			tetramino 
		}
    }
    pub fn new(columns: u8, rows: u8) -> GameState {
        let block = [[1, 1, 0, 0], [1, 1, 0, 0]]; // Block
        let t = [[1, 1, 1, 0], [0, 1, 0, 0]]; // T
        let l = [[0, 0, 1, 0], [1, 1, 1, 0]]; // L
        let reverse_l = [[1, 0, 0, 0], [1, 1, 1, 0]]; // Reverse L
        let straight = [[1, 1, 1, 1], [0, 0, 0, 0]]; // Straight
        let z = [[1, 1, 0, 0], [0, 1, 1, 0]]; // Z
        let s = [[0, 1, 1, 0], [1, 1, 0, 0]]; // S
        let tetraminos = [block, t, l, reverse_l, straight, z, s].map(Tetramino::new);
        let mut rng = rand::thread_rng();


		let player_tetramino = tetraminos[rng.gen_range(0..7)].clone();
		let player = Player {
			position: (((columns as f32 / 2.).ceil() as u8), rows),
			rotation: [[1,0], [0,1]],
			tetramino: player_tetramino
		};

        GameState {
			time: 0,
            tetraminos,
            player,
			rng,
            columns,
            rows,
        }
	}
	pub fn key_press(&mut self, key : VirtualKeyCode) {
		match key {
    		VirtualKeyCode::R => {
			    self.player.rotation = [
				    [self.player.rotation[0][1], -self.player.rotation[0][0]],
				    [self.player.rotation[1][1], -self.player.rotation[1][0]]
			    ]
		    },
			VirtualKeyCode::N => self.player = self.next_player(),
    		_ => (),
		}
	}
    pub fn update(&mut self, canvas: &mut Canvas, delta_t : u64) {
        canvas.draw_buffer(self.player.to_object_buffer().into_iter());
		self.time +=  delta_t;
		if self.time >= 10000 {
			if self.player.position.1 > 0 {

				self.player.position.1 -= 1;
			}
			self.time = 0;
		}  
    }
}
