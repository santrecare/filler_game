use rand::{Rng, thread_rng};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use super::board::Board;
use super::piece::Piece;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    pub board: Board,
    pub piece: Piece,
    pub player_one: String,
    pub player_two: String,
    pub current_player: i8,
    pub data: Vec<Value>,
}

pub fn set_game_state(player_one: String, player_two: String, board_size: usize, piece_size: usize) -> GameState {
    let mut rng = thread_rng();
    let mut piece = Piece::new(piece_size, 6);

    piece.generate_piece(
        rng.gen_range(0..piece_size) as usize,
        rng.gen_range(0..piece_size) as usize,
        0
    );
    let mut board = Board::new(board_size);
    board.set_start_position(
        rng.gen_range(0..board_size) as usize,
        rng.gen_range(0..board_size) as usize
    );
    GameState {
        board: board,
        piece: piece,
        player_one: player_one,
        player_two: player_two,
        current_player: 1,
        data: Vec::new(),
    }
}

pub fn play(game_state: &mut GameState) {
    let mut data = Vec::new();
    for py in (-1 * game_state.piece.size as isize)..game_state.board.size as isize {
        for px in (-1 * game_state.piece.size as isize)..game_state.board.size as isize {
            if game_state.board.is_valid_piece_coord(&game_state.piece, py, px, game_state.current_player) {
                data.push(json!({"pos": (py, px)}));
            }
        }
    }
    game_state.data = data.clone();
}
