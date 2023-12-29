use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

use super::TETRAMINO_TEMPLATES;

use super::Tetramino;

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
        let mut new_tetraminos = TETRAMINO_TEMPLATES.map(Tetramino::new).to_vec();
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
