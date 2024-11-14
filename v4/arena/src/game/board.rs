use serde::{Serialize, Deserialize};

use super::piece::Piece;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Board {
    pub size: usize,
    board: Vec<Vec<i8>>,
}


impl Board {
    pub fn new(size: usize) -> Self {
        let board = vec![vec![0; size]; size];
        Board { size, board }
    }

    fn is_valid_box(&self, y: isize, x: isize, player: i8) -> bool {
        y >= 0
            && x >= 0
            && y < self.size as isize
            && x < self.size as isize
            && (self.board[y as usize][x as usize] == 0
                || self.board[y as usize][x as usize] == player)
    }


    pub fn is_valid_piece_coord(&self, piece: &Piece, y: isize, x: isize, player: i8) -> bool {
        let mut neighbor_cnt = 0;

        for py in 0..piece.size {
            for px in 0..piece.size {
                if piece.piece[py][px].is_none() {
                    continue;
                }

                if !self.is_valid_box(y + py as isize, x + px as isize, player) {
                    return false;
                }

                if self.board[(y + py as isize) as usize][(x + px as isize) as usize] == player {
                    neighbor_cnt += 1;
                }
            }
        }
        if neighbor_cnt == 1 {
            return true;
        }
        return false;
    }

    pub fn set_piece(&mut self, piece: &Piece, y: isize, x: isize, player: i8) -> bool {
        for py in 0..piece.size {
            for px in 0..piece.size {
                if piece.piece[py][px].is_none() {
                    continue;
                }
                if self.board[(y + (py as isize)) as usize][(x + (px as isize)) as usize] <= 0 {
                    self.board[(y + (py as isize)) as usize][(x + (px as isize)) as usize] = player;
                }
            }
        }
        return true;
    }

    pub fn set_start_position(&mut self, y: usize, x: usize) {
        self.board[y][x] = 1;
        let y = if y > 0 { self.size - y } else { self.size - 1 };
        let x = if x > 0 { self.size - x } else { self.size - 1 };
        self.board[y][x] = 2;
    }

    pub fn display(&self) {
        for row in &self.board {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}
