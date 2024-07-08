from filler_arena import event
from os import path, remove
from json import dump


class Player:
    def __init__(self, player_name, arena_path, value):
        self.name = player_name
        self.player_move_path = path.join(arena_path, f'{player_name}.play.txt')
        self.arena_state_path = path.join(arena_path, f'{player_name}.state.json')
        self.value = value
        self.score = 0
        self.is_playing = True

    def play(self, piece, board):
        with open(self.arena_state_path, 'w') as f:
            dump({'piece': piece, 'board': board, 'player': self.value}, f)
        move = [None, None]
        while not event.is_set():
            if path.exists(self.player_move_path):
                try:
                    with open(self.player_move_path, 'r') as f:
                        move = [int(pos) for pos in f.read().split(',')]
                except:
                    continue
                remove(self.player_move_path)
                break
            event.wait(0.05)
        return move
