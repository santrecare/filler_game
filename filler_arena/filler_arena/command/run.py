import os
import asyncio
import argparse
import traceback
import websockets
from json import dumps

from filler_arena.board import Board
from filler_arena.arena import Arena
from filler_arena.player import Player
from filler_arena.piece import Piece


def get_args():
    parser = argparse.ArgumentParser(description='Filler Arena WebSocket Server')
    parser.add_argument('--player1', help='Name of the first player')
    parser.add_argument('--player2', help='Name of the second player')
    parser.add_argument('--arena_path', help='Path to the arena')
    parser.add_argument('--board_size', type=int, help='Size of the board')
    parser.add_argument('--piece_size', type=int, help='Size of the pieces')
    parser.add_argument('--piece_density', type=int, help='Density of the pieces')
    return parser.parse_args()


def clean_arena(path):
    if not os.path.exists(path):
        return
    for filename in os.listdir(path):
        file_path = os.path.join(path, filename)
        if os.path.isfile(file_path):
            os.unlink(file_path)


async def websocket_handler(websocket, path, args):
    try:
        arena = Arena(
            board=Board(size=args.board_size),
            first_player=Player(
                player_name=args.player1,
                arena_path=args.arena_path,
                value=1,
            ),
            second_player=Player(
                player_name=args.player2,
                arena_path=args.arena_path,
                value=2,
            ),
        )
        print(f"Game started between {args.player1} and {args.player2} with board size {args.board_size}")
        while arena.is_player_alive():
            arena.set_piece(Piece(
                size=args.piece_size,
                density=args.piece_density
            ))
            arena.play()
            await websocket.send(dumps(arena.get_arena_state()))
            await asyncio.sleep(0.01)
    except websockets.ConnectionClosed as e:
        print(f"Connection closed: {e}")
    except Exception as e:
        print(f"Unexpected error: {e}")
        traceback.print_exc()
    finally:
        print("WebSocket handler terminated.")

if __name__ == '__main__':
    args = get_args()
    clean_arena(args.arena_path)
    start_server = websockets.serve(
        lambda ws, path: websocket_handler(ws, path, args),
        '0.0.0.0',
        8765,
    )
    asyncio.get_event_loop().run_until_complete(start_server)
    asyncio.get_event_loop().run_forever()
