use rand::{Rng, thread_rng};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::board::{Board, PlayableCoordinates, BoardInfos};
use super::piece::{PieceInfos, Piece};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameInfos {
    piece_infos: PieceInfos,
    playable_coordinates: Vec<PlayableCoordinates>,
    board_infos: BoardInfos,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    pub board: Board,
    pub piece: Piece,
    pub player_one: String,
    pub player_two: String,
    pub current_player: i8,
    pub game_id: String,
    pub data: Option<GameInfos>,
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
        rng.gen_range(0..board_size - 1) as usize,
        rng.gen_range(0..board_size - 1) as usize
    );
    GameState {
        board: board,
        piece: piece,
        player_one: player_one,
        player_two: player_two,
        current_player: 1,
        game_id: Uuid::new_v4().to_string(),
        data: None,
    }
}

pub fn play(game_state: &mut GameState) {
    let mut playable_coordinates = Vec::<PlayableCoordinates>::new();
    for py in (-1 * game_state.piece.size as isize)..game_state.board.size as isize {
        for px in (-1 * game_state.piece.size as isize)..game_state.board.size as isize {
            if game_state.board.is_valid_piece_coord(&game_state.piece, py, px, game_state.current_player) {
                game_state.board.set_piece(
                    &game_state.piece, py, px, -1 * game_state.current_player
                );
                playable_coordinates.push(game_state.board.compute_playable_coordinates(&game_state.piece, py, px, game_state.current_player));
                game_state.board.set_piece(
                    &game_state.piece, py, px, 0
                );
            }
        }
    }
    game_state.data = Some(GameInfos {
        piece_infos: game_state.piece.compute_piece_infos(),
        playable_coordinates: playable_coordinates,
        board_infos: game_state.board.compute_board_infos(
            game_state.current_player,
            if game_state.current_player == 1 {2} else {1}
        ),
    });
}
