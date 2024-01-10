use std::{vec, rc::Rc};

use crate::{
    gui::{
        interface::{Canvas, Interface},
        systems::{ImageObject, SolidColorObject, TextObject},
        Rect,
    },
    include_png,
    vector2::{ToVec2, Vec2, Vector2}, core::rgb::Rgb,
};
use glium::glutin::event::VirtualKeyCode;
use glium::texture::SrgbTexture2d;

use super::{bag::Bag, Scene};
/// The size of a tetramino in the map
pub const SIZE: f32 = 5.;

/// Representation of a block of a tetramino in the stack
#[derive(Debug, Clone)]
struct Block {
    /// The color of the block
    color: Rgb,
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
    pub block_positions: [Option<Vec2>; 4],
    pub color: Rgb,
}

impl Tetramino {
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
    fn get_blocks(&self, position : Vector2<i16>) -> vec::IntoIter<Vector2<i16>>{
        self.block_positions
            .into_iter()
            .flatten()
            .map(|block| Vector2 {
                x: (position.x as f32 + block.x).floor() as i16,
                y: (position.y as f32 + block.y).floor() as i16,
            })
            .collect::<Vec<Vector2<i16>>>()
            .into_iter()
    }
    fn object_vec(&self, pos: Vector2<i16>) -> Vec<SolidColorObject> {
        self.get_blocks(pos)
            .map(|position| SolidColorObject {
                region: grid_region(position),
                color: self.color,
            })
            .collect()
    }
}

/// The informations of the player in the grid
#[derive(Debug)]
pub struct Player {
    tetramino: Tetramino,
    position: Vector2<i16>,
}

/// Get a object in the map based on the position in the grid and the color  
fn grid_region(position: Vector2<i16>) -> Rect {
    Rect {
        center: (position.to_vec2() + vec2!(0.5, 0.5)) * SIZE,
        size: vec2!(SIZE, SIZE),
    }
}

impl Player {
    /// Get a vector of objecto of  each block of the tetramino
    fn object_vec(&self) -> Vec<SolidColorObject> {
        self.tetramino.object_vec(self.position)
    }
    /// Returns a vector containing each position of the tetramino blocks relative to the origin
    fn get_blocks(&self) -> vec::IntoIter<Vector2<i16>> {
        self.tetramino.get_blocks(self.position)
    }
}
/// Represent the actual state of the game
#[derive(Debug)]
pub struct LevelScene {
    /// Actual player in the game  
    player: Player,
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
    score: u32,
    texture: Rc<SrgbTexture2d>,
    bag: Bag,
    loss : bool
}

impl LevelScene {
    pub fn world_region(&self) -> Rect {
        Rect {
            center: vec2!(self.columns as f32, self.rows as f32) * SIZE / 2.,
            size: vec2!(100., 100.),
        }
    }
    /// Generate the next player of the game
    fn next_player(&mut self) -> Player {
        let tetramino = self.bag.pop();

        Player {
            position: vec2!(
                ((self.columns as f32 / 2.).ceil() as i16) - 2,
                self.rows - 1
            ),
            tetramino,
        }
    }

    /// Create the game state
    pub fn new(interface: &Interface) -> LevelScene {
        let mut bag = Bag::new();
        let columns  = 10;
        let rows = 20;
        let tetramino =  bag.pop();
        let player = Player {
            position: vec2!((columns as f32 / 2.).ceil() as i16 - 2, rows - 2),
            tetramino,
        };

        LevelScene {
            time: 0,
            player,
            columns,
            rows,
            max_time: 1000000,
            stack: vec![],
            texture:  interface.create_texture(include_png!("../assets/brick.png")),
            bag,
            score: 0,
            loss: false
        }
    }
    /// Receives the keypress event
    pub fn key_down(&mut self, key: VirtualKeyCode) -> Scene {
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
        };
        Scene::LevelScene
    }

    /// Move the player to the end of the stack and put he in
    /// Moves the player to the position where he fits,
    /// going down until he finds a block or the floor
    /// and then puts him in the final position
    fn move_to_end(&mut self) {
        let mut i = 0;
        while self.translate_player(vec2!(0_i16, -1)) { 
            i += 1;
        }
        self.score += i*2;
        self.add_player_to_stack();
    }

    /// Restart the game
    ///
    /// Clears the stack and generates a new player
    fn restart(&mut self) {
        self.stack = vec![];
        self.player = self.next_player();
        self.score = 0;
        self.loss = true;
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
                self.score += 100;
            } else {
                i += 1;
            }
        }
    }
    /// Updates the game state and draws on the table
    pub fn update(&mut self, canvas: &mut Canvas, delta_t: u128) -> Scene {
        if self.loss {
            self.loss = false;
            return Scene::HomeScene;
        }
        for i in 0..self.columns {
            for j in 0..self.rows {
                let mut object = SolidColorObject {
                    region: grid_region(vec2!(i, j)),
                    color: Rgb::new(64, 64, 64),
                };
                object.region.size = object.region.size * 0.9;
                canvas.draw(object);
            }
        }
        
        for (i, row) in self.stack.iter().enumerate() {
            for (j,  op) in  row.iter().enumerate() {
                if let Some(Block {color})  = op{
                    canvas.draw(SolidColorObject {
                        region: grid_region(vec2!(j as i16, i as i16)),
                        color: *color,
                    });
                }
            }
        };

        canvas.draw(TextObject{
            color: Rgb::WHITE,
            font_size: 5., 
            position: vec2!(52., 100.),
            text: format!("score: {}", self.score)
        });
        
        canvas.draw(TextObject{
            color: Rgb::WHITE,
            font_size: 5., 
            position: vec2!(52., 80.),
            text: "next tetraminos".to_owned()
        });
        let nexts = self.bag.next_tetraminos();
        for (i, tetramino) in nexts.into_iter().enumerate() {
            let pos = vec2!(52,  50 - i as i16 * 15 );
            canvas.draw_iter(tetramino.object_vec(vec2!(0_i16,0)).into_iter().map(|mut obj| {
                obj.region.center += pos;
                obj
            }));
        }

        canvas.draw_iter(self.player.object_vec());

        self.time += delta_t;
        if self.time >= self.max_time {
            if !self.translate_player(vec2!(0_i16, -1)) {
                self.add_player_to_stack();
            }
            self.time -= self.max_time;
        }

        Scene::LevelScene
    }
    pub fn on_click(&mut self, position : Vec2) -> Scene {
        Scene::LevelScene
    }
}
