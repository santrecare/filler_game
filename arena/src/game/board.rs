use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::f64;

use super::piece::Piece;

#[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
enum CardinalPoints {
    North,
    South,
    East,
    West,
    Center,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CardinalPointsInfos {
    percent: f64,
    top_border: bool,
    bottom_border: bool,
    left_border: bool,
    right_border: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayableCoordinates {
    coordinates: Value,
    distance: f64,
    contact_zone_size: u32,
    top_border: bool,
    bottom_border: bool,
    left_border: bool,
    right_border: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoardInfos {
    fight_started: bool,
    player: HashMap<CardinalPoints, CardinalPointsInfos>,
    opponent: HashMap<CardinalPoints, CardinalPointsInfos>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Board {
    pub size: usize,
    board: Vec<Vec<i8>>,
}

fn compute_distance(point1: (f64, f64), point2: (f64, f64)) -> f64 {
    let (x1, y1) = point1;
    let (x2, y2) = point2;
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
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

    fn compute_smallest_dist(&self, y: usize, x: usize, player: i8) -> f64 {
        let mut distance: f64 = f64::MAX;
        let mut new_distance: f64 = 0.0;
        for by in 0..self.size {
            for bx in 0..self.size {
                if self.board[by][bx] != player && self.board[by][bx] > 0 {
                    new_distance = compute_distance((by as f64, bx as f64), (y as f64, x as f64));
                    if new_distance < distance {
                        distance = new_distance;
                    }
                }
            }
        }
        distance
    }

    fn compute_contact_zone_size(&self, y: usize, x: usize, player: i8) -> u32 {
        let mut res = 0;
        if y + 1 < self.size && self.board[y + 1][x] != player && self.board[y + 1][x] > 0 {
            res += 1;
        }
        if x + 1 < self.size && self.board[y][x + 1] != player && self.board[y][x + 1] > 0 {
            res += 1;
        }
        if y > 0 && self.board[y - 1][x] != player && self.board[y - 1][x] > 0 {
            res += 1;
        }
        if x > 0 && self.board[y][x - 1] != player && self.board[y][x - 1] > 0 {
            res += 1;
        }
        if y + 1 < self.size && x + 1 < self.size && self.board[y + 1][x + 1] != player && self.board[y + 1][x + 1] > 0 {
            res += 1;
        }
        if y > 0 && x > 0 && self.board[y - 1][x - 1] != player && self.board[y - 1][x - 1] > 0 {
            res += 1;
        }
        if y > 0 && x + 1 < self.size && self.board[y - 1][x + 1] != player && self.board[y - 1][x + 1] > 0 {
            res += 1;
        }
        if y + 1 < self.size && x > 0 && self.board[y + 1][x - 1] != player && self.board[y + 1][x - 1] > 0 {
            res += 1;
        }
        return res;
    }

    pub fn compute_playable_coordinates(&self, piece: &Piece, y: isize, x: isize, player: i8) -> PlayableCoordinates {
        let mut distance: f64 = 0.0;
        let mut contact_zone_size = 0;
        let mut top_border = false;
        let mut bottom_border = false;
        let mut left_border = false;
        let mut right_border = false;
        let mut by: usize = 0;
        let mut bx: usize = 0;
        for py in 0..piece.size {
            for px in 0..piece.size {
                if piece.piece[py][px].is_none() {
                    continue;
                }
                by = (y + (py as isize)) as usize;
                bx = (x + (px as isize)) as usize;
                distance += self.compute_smallest_dist(by, bx, player);
                contact_zone_size += self.compute_contact_zone_size(by, bx, player);
                if !top_border && by == 0 {
                    top_border = true;
                }
                if !bottom_border && by == self.size - 1 {
                    bottom_border = true;
                }
                if !left_border && bx == 0 {
                    left_border = true;
                }
                if !right_border && bx == self.size - 1 {
                    right_border = true;
                }
            }
        }
        PlayableCoordinates {
            coordinates: json!((y, x)),
            distance: distance,
            contact_zone_size: contact_zone_size,
            top_border: top_border,
            bottom_border: bottom_border,
            left_border: left_border,
            right_border: right_border,
        }
    }

    fn is_fight_started(&self, player: i8, opponent: i8) -> bool {
        for y in 0..self.size {
            for x in 0..self.size {
                if self.board[y][x] == player {
                    if (x + 1 < self.size && y + 1 < self.size && self.board[y + 1][x + 1] == opponent) ||
                        (x + 1 < self.size && self.board[y][x + 1] == opponent) ||
                        (y + 1 < self.size && self.board[y + 1][x] == opponent) {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn compute_cardinal_points_infos(&self, player: i8, start_y: usize, start_x: usize, end_y: usize, end_x: usize) -> CardinalPointsInfos {
        let mut occupation = 0;
        let mut top_border = false;
        let mut bottom_border = false;
        let mut left_border = false;
        let mut right_border = false;
        for y in start_y..end_y {
            for x in start_x..end_x {
                if self.board[y][x] == player {
                    occupation += 1;
                    if !top_border && y == start_y {
                        top_border = true;
                    }
                    if !bottom_border && y == end_y - 1 {
                        bottom_border = true;
                    }
                    if !left_border && x == start_x {
                        left_border = true;
                    }
                    if !right_border && x == end_x - 1 {
                        right_border = true;
                    }
                }
            }
        }
        let part_size = ((self.size as f64) / 3.0) * (self.size as f64) / 3.0;
        CardinalPointsInfos {
            percent: (occupation * 100) as f64 / part_size,
            top_border: top_border,
            bottom_border: bottom_border,
            left_border: left_border,
            right_border: right_border,
        }
    }

    pub fn compute_board_infos(&self, ply: i8, opt: i8) -> BoardInfos {
        let part_size: usize = self.size / 3;
        let mut player = HashMap::<CardinalPoints, CardinalPointsInfos>::new();
        player.insert(CardinalPoints::North, self.compute_cardinal_points_infos(ply, 0, part_size, part_size, part_size * 2).clone());
        player.insert(CardinalPoints::South, self.compute_cardinal_points_infos(ply, part_size * 2, part_size, self.size, part_size * 2).clone());
        player.insert(CardinalPoints::East, self.compute_cardinal_points_infos(ply, part_size, part_size * 2, part_size * 2, self.size).clone());
        player.insert(CardinalPoints::West, self.compute_cardinal_points_infos(ply, part_size, 0, part_size * 2, part_size).clone());
        player.insert(CardinalPoints::NorthEast, self.compute_cardinal_points_infos(ply, 0, part_size * 2, part_size, self.size).clone());
        player.insert(CardinalPoints::NorthWest, self.compute_cardinal_points_infos(ply, 0, 0, part_size, part_size).clone());
        player.insert(CardinalPoints::SouthEast, self.compute_cardinal_points_infos(ply, part_size * 2, part_size * 2, self.size, self.size).clone());
        player.insert(CardinalPoints::SouthWest, self.compute_cardinal_points_infos(ply, part_size * 2, 0, self.size, part_size).clone());
        player.insert(CardinalPoints::Center, self.compute_cardinal_points_infos(ply, part_size, part_size, part_size * 2, part_size * 2).clone());
        let mut opponent = HashMap::<CardinalPoints, CardinalPointsInfos>::new();
        opponent.insert(CardinalPoints::North, self.compute_cardinal_points_infos(opt, 0, part_size, part_size, part_size * 2).clone());
        opponent.insert(CardinalPoints::South, self.compute_cardinal_points_infos(opt, part_size * 2, part_size, self.size, part_size * 2).clone());
        opponent.insert(CardinalPoints::East, self.compute_cardinal_points_infos(opt, part_size, part_size * 2, part_size * 2, self.size).clone());
        opponent.insert(CardinalPoints::West, self.compute_cardinal_points_infos(opt, part_size, 0, part_size * 2, part_size).clone());
        opponent.insert(CardinalPoints::NorthEast, self.compute_cardinal_points_infos(opt, 0, part_size * 2, part_size, self.size).clone());
        opponent.insert(CardinalPoints::NorthWest, self.compute_cardinal_points_infos(opt, 0, 0, part_size, part_size).clone());
        opponent.insert(CardinalPoints::SouthEast, self.compute_cardinal_points_infos(opt, part_size * 2, part_size * 2, self.size, self.size).clone());
        opponent.insert(CardinalPoints::SouthWest, self.compute_cardinal_points_infos(opt, part_size * 2, 0, self.size, part_size).clone());
        opponent.insert(CardinalPoints::Center, self.compute_cardinal_points_infos(opt, part_size, part_size, part_size * 2, part_size * 2).clone());
        BoardInfos {
            fight_started: self.is_fight_started(ply, opt),
            player: player,
            opponent: opponent,
        }
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
