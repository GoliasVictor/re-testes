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

#[derive(Debug, Clone)]
struct Block {
  color: (i16, i16, i16)
}

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
        
        return Tetramino {
            color: template.color,
            block_positions: block_positions,
        };
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
    return Object {
        format: Rect {
            center: (position.to_vec2() + vec2!(0.5, 0.5))*SIZE,
            size: vec2!(SIZE, SIZE),
        },
        color: f_color,
    };
}

impl Player {
    fn to_object_buffer(&self) -> Vec<Object> {
        self.tetramino.block_positions.into_iter().flatten().map(|block| {
            let pos = self.position.to_vec2() + block;
            to_object(vec2!(pos.x as i16, pos.y as i16), self.tetramino.color)
        }).collect()
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
    stack: Vec<Vec<Option<Block>>>
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
            player: player,
			rng: rng,
            columns: columns,
            rows: rows,
            max_time: 1000000,
            stack: vec!{}
        }
	}
	pub fn key_down(&mut self, key : VirtualKeyCode) {
		match key {
    		VirtualKeyCode::Up => {
			    self.rotate_player();
		    },
            VirtualKeyCode::Down => {
                self.translate_player(vec2!(0_i16, -1));
		    },
    		VirtualKeyCode::W => {
				self.player.position.y += 1;
		    },
            VirtualKeyCode::Right => {
                self.translate_player(vec2!(1_i16, 0));
		    },
            VirtualKeyCode::Left => {
                self.translate_player(vec2!(-1_i16, 0));
		    },
			VirtualKeyCode::N => self.player = self.next_player(),
    		_ => (),
		}
	}
    pub fn is_valid_player_position(&self, pos: Vector2<i16>) -> bool{
        if  0  > pos.x  || pos.x  >= self.columns {
            return false;
        }
        if 0  > pos.y  || pos.y >= self.rows {
            return false;
        }
        
        if let Some(row) = self.stack.get(pos.y as usize) {
            if row[pos.x as usize].is_some(){
                return false
            }
        } 
        true 
        
    }   
    pub fn translate_player(&mut self, delta : Vector2<i16>) -> bool {
        let can_move  =  self.player.tetramino.block_positions.iter().flatten().all(|block|{
            let new_pos = self.player.position  + delta +  vec2!(block.x as i16, block.y as i16);
            self.is_valid_player_position(new_pos)
        });
        if can_move {
            self.player.position +=  delta;
        }
        return can_move;
    }
    fn rotate_player(&mut self) {
        let center = self.player.tetramino.get_center();
        let new_blocks =    self.player.tetramino.block_positions.map(|op| op.map(|block| {
            let relative_position = center - block;
        
            let mut x_multiplier = 1.;
            let mut y_multiplier = 1.;

            if relative_position.x <= 0.0 { y_multiplier = -1. }
            if relative_position.y >= 0.0 { x_multiplier = -1. }

            Vec2 {
                x: center.x + (relative_position.y.abs() * x_multiplier as f32),
                y: center.y + (relative_position.x.abs() * y_multiplier as f32)
            }
        }));
        let can_rotate = |b : Vec2| {
            let pos =  self.player.position + vec2!(b.x.floor() as i16, b.y.floor() as i16);
            self.is_valid_player_position(pos)
        };
        if new_blocks.into_iter().flatten().all(can_rotate) {
            self.player.tetramino.block_positions =  new_blocks;
        }
        
    }
    pub fn add_player_to_stack(&mut self){
        self.player.tetramino.block_positions.into_iter().flatten().for_each(|b|{
            let pos = Vector2::<usize> {
                x: (self.player.position.x  as f32 + b.x.floor()) as usize,
                y: (self.player.position.y as f32 + b.y.floor()) as usize
            };
            while self.stack.len() <= pos.y{
                self.stack.push(vec![None; self.columns as usize]);
            }
            self.stack[pos.y][pos.x] = Some(Block {
                color: self.player.tetramino.color
            })
        });
        self.player = self.next_player();
        let mut i = 0;
        while i < self.stack.len() {
            if self.stack[i].iter().all(Option::is_some) {
                self.stack.remove(i);
            } else {
                i += 1;
            }
        }
    }
    pub fn update(&mut self, canvas: &mut Canvas, delta_t : u128) {


        for i in 0..self.columns {
            for j in 0..self.rows {
                let mut obj = to_object(vec2!(i,j), (64,64,64));
                obj.format.size =  obj.format.size  * 0.9;
                canvas.draw_obj(&obj);
            }
        }

        canvas.draw_buffer(self.player.to_object_buffer().into_iter());
        let buffer = self.stack.iter().enumerate().flat_map(|(i,row)|{
            row.iter().enumerate().flat_map(move |(j,op)| {
                op.as_ref().map(|b| to_object(vec2!(j as i16, i as i16), b.color))
            })
        });
        canvas.draw_buffer(buffer);
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
            if !self.translate_player(vec2!(0_i16, -1)) {
                self.add_player_to_stack();   
            }
            self.time -= self.max_time;
		}  
    }
}
