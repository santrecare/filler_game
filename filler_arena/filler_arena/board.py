from copy import deepcopy
from random import randint


class Board:
    def __init__(self, size, start_pos_count):
        self.size = size
        self.board = [[None for _ in range(0, self.size)] for _ in range(0, self.size)]
        self.set_start_position_count(start_pos_count)

    def __is_piece_insertable(self, x, y):
        if x < 0 or x >= self.size or y < 0 or y >= self.size or self.board[y][x]:
            return False
        return True

    def __is_piece_neighbor(self, x, y, player):
        if (x + 1 < self.size and self.board[y][x + 1] == player)\
                or (y + 1 < self.size and self.board[y + 1][x] == player)\
                or (x - 1 >= 0 and self.board[y][x - 1] == player)\
                or (y - 1 >= 0 and self.board[y - 1][x] == player):
            return True
        return False

    def place_piece(self, piece, x, y, player):
        is_neighbor = False
        board = deepcopy(self.board)
        board = [[box * -1 if box and box < 0 else box for box in row] for row in board]
        for py in range(0, len(piece)):
            for px in range(0, len(piece)):
                if not piece[py][px]:
                    continue
                if not self.__is_piece_insertable(px + x, py + y):
                    return False
                board[y + py][x + px] = -player
                if not is_neighbor:
                    is_neighbor = self.__is_piece_neighbor(x + px, y + py, player)
        if is_neighbor:
            self.board = board
        return is_neighbor

    def set_start_position_count(self, start_pos_count):
        for _ in range(0, start_pos_count):
            starter_y = randint(0, self.size - 1)
            starter_x = randint(0, self.size - 1)
            self.board[starter_y][starter_x] = 1
            self.board[-starter_y][-starter_x] = 2

    def description(self):
        board = ''
        for y in range(0, self.size):
            for x in range(0, self.size):
                board += f'{self.board[y][x] if self.board[y][x] else "."}'
            board += '\n'
        print(f'size: {self.size}\nboard:\n{board}')
