from os import environ, path, remove
from threading import Event
from random import randint
from json import load


class Player:
    def __init__(self, board_size, piece_size):
        self.board_size = board_size
        self.piece_size = piece_size

    def play(self, player, piece, board):
        coords = self.__find_insert_coords(player, piece, board)
        return coords[randint(0, len(coords) - 1)] if coords else [-1, -1]

    def __is_piece_insertable(self, board, x, y):
        if x < 0 or x >= self.board_size or y < 0 or y >= self.board_size or board[y][x]:
            return False
        return True

    def __is_piece_neighbor(self, player, board, x, y):
        if x < self.board_size and x >= 0 and y < self.board_size and y >= 0\
                and ((x + 1 < self.board_size and board[y][x + 1] == player)
                or (y + 1 < self.board_size and board[y + 1][x] == player)
                or (x - 1 >= 0 and board[y][x - 1] == player)
                or (y - 1 >= 0 and board[y - 1][x] == player)):
            return True
        return False

    def __is_valid_piece_coord(self, player, piece, board, x, y):
        """Verify if a piece can be placed on the board with specific coord

        Return
        ------
        bool
        """
        is_neighbor = False
        for py in range(0, self.piece_size):
            for px in range(0, self.piece_size):
                if not piece[py][px]:
                    continue
                if not self.__is_piece_insertable(board, px + x, py + y):
                    return False
                if not is_neighbor:
                    is_neighbor = self.__is_piece_neighbor(
                        player, board, px + x, py + y
                    )
        return is_neighbor

    def __find_insert_coords(self, player, piece, board):
        """Loop over the board and test all coords to place the piece.

        Return
        ------
        list of tuple, contains all valid piece coords
        """
        valid_piece_coords = []
        y = -self.piece_size
        while y < self.board_size:
            x = -self.piece_size
            while x < self.board_size:
                if self.__is_valid_piece_coord(player, piece, board, x, y):
                    valid_piece_coords.append((x, y))
                x += 1
            y += 1
        return valid_piece_coords


if __name__ == '__main__':
    event = Event()
    state_path = path.join(
        environ['ARENA_PATH'], f'{environ["PLAYER_NAME"]}.state.json')
    play_path = path.join(
        environ['ARENA_PATH'], f'{environ["PLAYER_NAME"]}.play.txt')
    player = Player(
        board_size=int(environ['BOARD_SIZE']),
        piece_size=int(environ['PIECE_SIZE']),
    )
    while not event.is_set():
        if path.exists(state_path):
            try:
                with open(state_path, 'rb') as file:
                    game_state = load(file)
            except:
                continue
            remove(state_path)
            pos = player.play(
                player=game_state['player'],
                piece=game_state['piece'],
                board=game_state['board'],
            )
            with open(play_path, 'w') as file:
                file.write(','.join(map(str, pos)))
        event.wait(0.05)
