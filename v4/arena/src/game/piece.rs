use rand::{Rng, thread_rng};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Piece {
    pub size: usize,
    pub density: u32,
    pub piece: Vec<Vec<Option<char>>>,
}

impl Piece {
    pub fn new(size: usize, density: u32) -> Self {
        let piece = vec![vec![None; size]; size];
        Piece {
            size,
            density,
            piece,
        }
    }

    pub fn generate_piece(&mut self, x: usize, y: usize, mut piece_size: usize) -> usize {
        let mut rng = thread_rng();
        self.piece[y][x] = Some('*');
        piece_size += 1;

        if x + 1 < self.size && rng.gen_range(0..self.density) == 1 && self.piece[y][x + 1].is_none() {
            piece_size = self.generate_piece(x + 1, y, piece_size); // right
        }
        if y + 1 < self.size && rng.gen_range(0..self.density) == 1 && self.piece[y + 1][x].is_none() {
            piece_size = self.generate_piece(x, y + 1, piece_size); // up
        }
        if x > 0 && rng.gen_range(0..self.density) == 1 && self.piece[y][x - 1].is_none() {
            piece_size = self.generate_piece(x - 1, y, piece_size); // left
        }
        if y > 0 && rng.gen_range(0..self.density) == 1 && self.piece[y - 1][x].is_none() {
            piece_size = self.generate_piece(x, y - 1, piece_size); // down
        }
        if x + 1 < self.size && y + 1 < self.size && rng.gen_range(0..self.density) == 1 && self.piece[y + 1][x + 1].is_none() {
            piece_size = self.generate_piece(x + 1, y + 1, piece_size); // right + up
        }
        if y + 1 < self.size && x > 0 && rng.gen_range(0..self.density) == 1 && self.piece[y + 1][x - 1].is_none() {
            piece_size = self.generate_piece(x - 1, y + 1, piece_size); // left + up
        }
        if x > 0 && y > 0 && rng.gen_range(0..self.density) == 1 && self.piece[y - 1][x - 1].is_none() {
            piece_size = self.generate_piece(x - 1, y - 1, piece_size); // left + down
        }
        if y > 0 && x + 1 < self.size && rng.gen_range(0..self.density) == 1 && self.piece[y - 1][x + 1].is_none() {
            piece_size = self.generate_piece(x + 1, y - 1, piece_size); // right + down
        }
        if piece_size < 2 {
            piece_size = self.generate_piece(x, y, piece_size - 1)
        }
        piece_size
    }

    pub fn display(&self) {
        for row in &self.piece {
            for cell in row {
                match cell {
                    Some(c) => print!("{}", c),
                    None => print!(" "),
                }
            }
            println!();
        }
    }
}
