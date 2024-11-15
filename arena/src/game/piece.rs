use rand::{Rng, thread_rng};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PieceInfos {
    witdh: usize,
    height: usize,
    density: f64,
    is_horizontal: bool,
    is_vertical: bool,
}

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

    pub fn compute_piece_infos(&self) -> PieceInfos {
        let mut points: Vec<(usize, usize)> = Vec::new();
        for i in 0..self.size {
            for j in 0..self.size {
                if self.piece[i][j] == Some('*') {
                    points.push((i, j));
                }
            }
        }
        let mut min_x = self.size;
        let mut max_x = 0;
        let mut min_y = self.size;
        let mut max_y = 0;
        for &(x, y) in &points {
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }
        let piece_height = max_x - min_x + 1;
        let piece_width = max_y - min_y + 1;

        PieceInfos {
            witdh: piece_width,
            height: piece_height,
            density: (points.len() * 100) as f64 / (piece_height * piece_width) as f64,
            is_horizontal: piece_height <= piece_width,
            is_vertical: piece_height >= piece_width,
        }
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
