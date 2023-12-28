//! Module containing the specific mechanics of the Tetris game, such as receiving events, etc.
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{rc::Rc, vec};

use crate::{
    gui::{
        interface::{Canvas, Interface},
        systems::{ImageObject, SolidColorObject},
        ObjectWrapper, Rect,
    },
    include_png,
    vector2::{ToVec2, Vec2, Vector2},
};
use glium::glutin::event::VirtualKeyCode;
use glium::texture::SrgbTexture2d;
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Debug)]
/// Template to create a new tetramino
pub struct TetraminoTemplate {
    /// Binary number for the blocks, first four represent top row, last four represent bottom
    blocks: i16,
    /// The color of the tetramino
    color: (i16, i16, i16),
}

/// List of the default tretraminos
const TETRAMINO_TEMPLATES: [TetraminoTemplate; 7] = [
    TetraminoTemplate {
        blocks: 0b11001100,
        color: (241, 196, 15),
    }, // Square
    TetraminoTemplate {
        blocks: 0b11100100,
        color: (142, 68, 173),
    }, // T
    TetraminoTemplate {
        blocks: 0b00101110,
        color: (230, 126, 34),
    }, // L
    TetraminoTemplate {
        blocks: 0b10001110,
        color: (41, 128, 185),
    }, // Reverse L
    TetraminoTemplate {
        blocks: 0b11110000,
        color: (93, 173, 226),
    }, // Straight
    TetraminoTemplate {
        blocks: 0b11000110,
        color: (231, 76, 60),
    }, // Z
    TetraminoTemplate {
        blocks: 0b01101100,
        color: (46, 204, 113),
    }, // S
];

/// Bag in which all seven tetraminoes are located and then suffled
pub struct Bag {
    list: Vec<TetraminoTemplate>,
}

impl Bag {
    /// Create unshuffled Bag
    pub fn new() -> Self {
        let mut bag = Self {
            list: TETRAMINO_TEMPLATES.to_vec(),
        };
        bag.populate();
        bag
    }

    /// Populate bag
    pub fn populate(&mut self) -> () {
        let mut rng = thread_rng();
        self.list = TETRAMINO_TEMPLATES.to_vec();
        self.list.shuffle(&mut rng);
    }

    /// Pop piece from bag
    pub fn pop(&mut self) -> TetraminoTemplate {
        match self.list.pop() {
            Some(val) => val,
            None => {
                self.populate();
                self.list.pop().unwrap()
            }
        }
    }
}

/// Representation of a block of a tetramino in the stack
#[derive(Debug, Clone)]
struct Block {
    /// The color of the block
    color: (i16, i16, i16),
}

/// The tetramino in the game
#[derive(Clone, Debug)]
pub struct Tetramino {
    /// Vector of positions of the tetramino in relation of the center of the tetramino
    ///
    /// The position can be a fractional value such as 0.25 to keep the center of mass stable,
    /// however when converting to an integer it is necessary to apply the floor function
    ///
    /// **Warning**: Do not convert to integer by just applying ```as i16```, this is like applying `.trunc` where `-0.25` becomes `0` instead of `-1`, which can cause errors
    block_positions: [Option<Vec2>; 4],
    color: (i16, i16, i16),
}

impl Tetramino {
    /// Create a new tetramino based in a template
    fn new(template: TetraminoTemplate) -> Tetramino {
        let mut block_positions: [Option<Vec2>; 4] = [Some(vec2!(0.0, 0.0)); 4];
        let mut i = 0;
        for x in 0..4 {
            for y in 0..2 {
                if template.blocks & (1 << (x + (4 * y))) != 0 {
                    block_positions[i] = Some(vec2!(x as f32, y as f32));
                    i += 1;
                }
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
}

/// The informations of the player in the grid
#[derive(Debug)]
pub struct Player {
    tetramino: Tetramino,
    position: Vector2<i16>,
}
/// The size of a tetramino in the map
pub const SIZE: f32 = 5.;

/// Get a object in the map based on the position in the grid and the color  
fn to_object(position: Vector2<i16>, color: (i16, i16, i16)) -> ObjectWrapper {
    let mut f_color: [f32; 3] = [0.0; 3];
    f_color[0] = color.0 as f32 / 255.0;
    f_color[1] = color.1 as f32 / 255.0;
    f_color[2] = color.2 as f32 / 255.0;
    ObjectWrapper::SolidColorObject(SolidColorObject {
        format: Rect {
            center: (position.to_vec2() + vec2!(0.5, 0.5)) * SIZE,
            size: vec2!(SIZE, SIZE),
        },
        color: f_color,
    })
}

impl Player {
    /// Get a vector of objecto of  each block of the tetramino
    fn to_object_buffer(&self) -> Vec<ObjectWrapper> {
        self.get_blocks()
            .map(|block| to_object(block, self.tetramino.color))
            .collect()
    }
    /// Returns a vector containing each position of the tetramino blocks relative to the origin
    fn get_blocks(&self) -> vec::IntoIter<Vector2<i16>> {
        self.tetramino
            .block_positions
            .into_iter()
            .flatten()
            .map(|block| Vector2 {
                x: (self.position.x as f32 + block.x).floor() as i16,
                y: (self.position.y as f32 + block.y).floor() as i16,
            })
            .collect::<Vec<Vector2<i16>>>()
            .into_iter()
    }
}
/// Represent the actual state of the game
#[derive(Debug)]
pub struct GameState {
    /// Actual player in the game  
    player: Player,
    /// The random number generator  
    rng: ThreadRng,
    /// Number of columns in the grid
    pub columns: i16,
    /// Number of rows in the grid
    pub rows: i16,
    /// Time between the last update that moved the player down and the current one
    time: u128,
    /// Time between player update and another
    max_time: u128,
    /// Vector of lines of blocks on the grid
    stack: Vec<Vec<Option<Block>>>,
    texture: Rc<SrgbTexture2d>,
}

impl GameState {
    /// Generate the next player of the game
    fn next_player(&mut self) -> Player {
        let tetramino = Tetramino::new(TETRAMINO_TEMPLATES[self.rng.gen_range(0..7)].clone());

        Player {
            position: vec2!(
                ((self.columns as f32 / 2.).ceil() as i16) - 2,
                self.rows - 1
            ),
            tetramino,
        }
    }

    /// Create the game state
    pub fn new(columns: i16, rows: i16, interface: &Interface) -> GameState {
        let mut rng = rand::thread_rng();

        let tetramino = Tetramino::new(TETRAMINO_TEMPLATES[rng.gen_range(0..7)].clone());
        let player = Player {
            position: vec2!((columns as f32 / 2.).ceil() as i16 - 2, rows - 2),
            tetramino,
        };

        GameState {
            time: 0,
            player,
            rng,
            columns,
            rows,
            max_time: 1000000,
            stack: vec![],
            texture: interface.create_texture(include_png!("./assets/brick.png")),
        }
    }
    /// Receives the keypress event
    pub fn key_down(&mut self, key: VirtualKeyCode) {
        match key {
            VirtualKeyCode::Up => {
                self.rotate_player();
            }
            VirtualKeyCode::Space => self.move_to_end(),
            VirtualKeyCode::Down => {
                self.translate_player(vec2!(0_i16, -1));
            }
            VirtualKeyCode::W => {
                self.player.position.y += 1;
            }
            VirtualKeyCode::Right => {
                self.translate_player(vec2!(1_i16, 0));
            }
            VirtualKeyCode::Left => {
                self.translate_player(vec2!(-1_i16, 0));
            }
            VirtualKeyCode::R => self.restart(),
            _ => (),
        }
    }

    /// Move the player to the end of the stack and put he in
    /// Moves the player to the position where he fits,
    /// going down until he finds a block or the floor
    /// and then puts him in the final position
    fn move_to_end(&mut self) {
        while self.translate_player(vec2!(0_i16, -1)) {}
        self.add_player_to_stack();
    }

    /// Restart the game
    ///
    /// Clears the stack and generates a new player
    fn restart(&mut self) {
        self.stack = vec![];
        self.player = self.next_player()
    }

    /// Checks whether a player block can be in the received position
    ///
    /// returns false if it is outside the sides,
    /// or lower than it should be,
    /// or in a stack block position otherise returns true
    pub fn is_valid_player_position(&self, pos: Vector2<i16>) -> bool {
        if 0 > pos.x || pos.x >= self.columns {
            return false;
        }
        if 0 > pos.y {
            return false;
        }

        if let Some(row) = self.stack.get(pos.y as usize) {
            if row[pos.x as usize].is_some() {
                return false;
            }
        }
        true
    }

    /// Move the posistion of the basead on delta if the new pos is valid
    ///
    /// It receives a difference (delta),
    /// if each block in relation to the new position is in a valid position,
    /// the position is replaced otherwise, nothing is done
    /// returns whether it was moved or not
    pub fn translate_player(&mut self, delta: Vector2<i16>) -> bool {
        let can_move = self
            .player
            .get_blocks()
            .all(|block| self.is_valid_player_position(delta + block));
        if can_move {
            self.player.position += delta;
        }
        can_move
    }

    /// Rotates the player's tetramino blocks if is possible
    /// Generates new tetramino positions, rotating 90 degrees relative to the center
    /// if the new positions are invalid, do nothing, otherwise the position will be the rotated position
    fn rotate_player(&mut self) {
        let center = self.player.tetramino.get_center();
        let new_blocks = self.player.tetramino.block_positions.map(|op| {
            op.map(|block| {
                let relative_position = center - block;
                center + vec2!(relative_position.y, -relative_position.x)
            })
        });
        let can_rotate = |b: Vec2| {
            let pos = self.player.position + vec2!(b.x.floor() as i16, b.y.floor() as i16);
            self.is_valid_player_position(pos)
        };
        if new_blocks.into_iter().flatten().all(can_rotate) {
            self.player.tetramino.block_positions = new_blocks;
        }
    }
    /// Method to add the player to the block stack
    ///
    /// checks if the player fits in the grid, if not, restarts the game,
    /// and then adds each player's block to the stack and generates a new one adds a new player,
    /// and removes the lines where it is filled
    pub fn add_player_to_stack(&mut self) {
        let max_height = self.player.get_blocks().map(|b| b.y).max().unwrap();
        if max_height >= self.rows {
            self.restart();
            return;
        }

        while self.stack.len() <= max_height as usize {
            self.stack.push(vec![None; self.columns as usize]);
        }
        for block in self.player.get_blocks() {
            self.stack[block.y as usize][block.x as usize] = Some(Block {
                color: self.player.tetramino.color,
            })
        }
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
    /// Updates the game state and draws on the table
    pub fn update(&mut self, canvas: &mut Canvas, delta_t: u128) {
        for i in 0..self.columns {
            for j in 0..self.rows {
                let mut wrap = to_object(vec2!(i, j), (64, 64, 64));
                if let ObjectWrapper::SolidColorObject(object) = &mut wrap {
                    object.format.size = object.format.size * 0.9;
                }

                canvas.draw_obj(&wrap);
            }
        }

        canvas.draw_buffer(self.player.to_object_buffer().into_iter());

        let buffer = self
            .stack
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().flat_map(move |(j, op)| {
                    op.as_ref()
                        .map(|b| to_object(vec2!(j as i16, i as i16), b.color))
                })
            })
            .map(|wrap| {
                if let ObjectWrapper::SolidColorObject(object) = wrap {
                    ObjectWrapper::ImageObject(ImageObject {
                        region: object.format,
                        texture: self.texture.clone(),
                    })
                } else {
                    wrap
                }
            });
        canvas.draw_buffer(buffer);

        self.time += delta_t;
        if self.time >= self.max_time {
            if !self.translate_player(vec2!(0_i16, -1)) {
                self.add_player_to_stack();
            }
            self.time -= self.max_time;
        }
    }
}
