use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::vector2::Vec2;

use super::level_scene::Tetramino;


#[derive(Debug)]
/// Template to create a new tetramino
struct TetraminoTemplate {
    /// Binary number for the blocks, first four represent top row, last four represent bottom
    blocks: i16,
    /// The color of the tetramino
    color: (i16, i16, i16),
}

impl TetraminoTemplate {
    /// Create a new tetramino based in a template
    fn build(self) -> Tetramino {
        let mut block_positions: [Option<Vec2>; 4] = [Some(vec2!(0.0, 0.0)); 4];
        let mut i = 0;
        for x in 0..4 {
            for y in 0..2 {
                if self.blocks & (1 << (x + (4 * y))) != 0 {
                    block_positions[i] = Some(vec2!(x as f32, y as f32));
                    i += 1;
                }
            }
        }

        Tetramino {
            color: self.color,
            block_positions,
        }
    }
    
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
#[derive(Debug)]
pub struct Bag {
    pub list: Vec<Tetramino>,
    /// The random number generator  
    pub rng: ThreadRng,
}

impl Bag {
    /// Create unshuffled Bag
    pub fn new() -> Self {
        let mut bag = Self {
            list: Vec::default(),
            rng: thread_rng(),
        };
        bag.populate();
        bag
    }

    /// Populate bag
    pub fn populate(&mut self) {
        let mut new_tetraminos = TETRAMINO_TEMPLATES.map(TetraminoTemplate::build).to_vec();
        new_tetraminos.shuffle(&mut self.rng);
        self.list.extend(new_tetraminos);
    }

    /// Pop piece from bag
    pub fn pop(&mut self) -> Tetramino {
        if self.list.len() < 3 {
            self.populate();
        }
        self.list.pop().unwrap()
    }
    pub fn next_tetraminos(&self) -> [&Tetramino; 3] {
        [&self.list[0], &self.list[1], &self.list[2]]
    }
}
