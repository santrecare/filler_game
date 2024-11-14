import argparse
from os import environ, path, remove
from threading import Event
from json import load

from python_player.board import Board
from python_player.piece import Piece


def get_args():
    parser = argparse.ArgumentParser(description='Filler Player')
    parser.add_argument('--arena_path', help='Path to the arena')
    parser.add_argument('--player-name', help='Player name')
    return parser.parse_args()

def play(board, piece):
    pos = (-42, -42)
    best_board_eval = float('-inf')
    for by in range(-piece.size, board.size):
        for bx in range(-piece.size, board.size):
            if not board.is_valid_piece_coord(piece, by, bx):
                continue
            board_copy = Board(board=board.board)
            board_copy.set_piece(piece, by, bx)
            board_eval = board_copy.eval()
            if best_board_eval < board_eval:
                best_board_eval = board_eval
                pos = (by, bx)
    return pos


if __name__ == '__main__':
    event = Event()
    state_path = path.join(
        environ['ARENA_PATH'], f'{environ["PLAYER_NAME"]}.state.json')
    play_path = path.join(
        environ['ARENA_PATH'], f'{environ["PLAYER_NAME"]}.play.txt')
    while not event.is_set():
        if path.exists(state_path):
            try:
                with open(state_path, 'rb') as file:
                    game_state = load(file)
            except:
                continue
            remove(state_path)
            pos = play(
                board=Board(board=game_state['board']),
                piece=Piece(
                    piece=game_state['piece'],
                    size=len(game_state['piece']),
                    player=game_state['player'],
                ),
            )
            with open(play_path, 'w') as file:
                file.write(','.join(map(str, pos)))
        event.wait(0.05)
