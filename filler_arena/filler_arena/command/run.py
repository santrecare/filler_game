import asyncio
import websockets
from json import dumps
from os import environ

from filler_arena.board import Board
from filler_arena.piece import Piece
from filler_arena.player import Player


def play(player, piece, board):
    if not player.is_playing:
        return
    x, y = player.play(piece.piece, board.board)
    if (x is not None or y is not None)\
            and board.place_piece(piece.piece, x, y, player.value):
        player.score += 1
    else:
        player.is_playing = False


async def websocket_handler(websocket, path):
    board = Board(
        size=int(environ['BOARD_SIZE']),
        start_pos_count=int(environ.get('START_POSITION_COUNT', 1)))
    player1 = Player(
        player_name=environ['PLAYER1'],
        arena_path=environ['ARENA_PATH'],
        value=1,
    )
    player2 = Player(
        player_name=environ['PLAYER2'],
        arena_path=environ['ARENA_PATH'],
        value=2,
    )
    while player1.is_playing or player2.is_playing:
        piece = Piece(int(environ['PIECE_SIZE']))
        play(player1, piece, board)
        await websocket.send(dumps({
            'board': board.board,
            'piece': piece.piece,
            'player1': {
                'name': player1.name,
                'value': player1.value,
                'score': player1.score,
            },
            'player2': {
                'name': player2.name,
                'value': player2.value,
                'score': player2.score,
            }
        }))
        play(player2, piece, board)
        await websocket.send(dumps({
            'board': board.board,
            'piece': piece.piece,
            'player1': {
                'name': player1.name,
                'value': player1.value,
                'score': player1.score,
            },
            'player2': {
                'name': player2.name,
                'value': player2.value,
                'score': player2.score,
            }
        }))

        await asyncio.sleep(0.01)


if __name__ == '__main__':
    start_server = websockets.serve(websocket_handler, '0.0.0.0', 8765)
    asyncio.get_event_loop().run_until_complete(start_server)
    asyncio.get_event_loop().run_forever()
