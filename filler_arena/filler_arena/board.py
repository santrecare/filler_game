from copy import deepcopy
from random import randint


class Board:
    def __init__(self, size):
        self.size = size
        self.board = [[None for _ in range(0, self.size)] for _ in range(0, self.size)]

    def __is_valid_box(self, y, x, pval):
        return (x >= 0 and x < self.size and y >= 0 and y < self.size
            and (not self.board[y][x] or self.board[y][x] == pval))

    def insert_piece(self, piece, y, x, player):
        neighbor_cnt = 0
        board = deepcopy(self.board)
        board = [[box * -1 if box and box < 0 else box for box in row] for row in board]
        for py in range(0, piece.size):
            for px in range(0, piece.size):
                if not piece.piece[py][px]:
                    continue
                if not self.__is_valid_box(py + y, px + x, player.value):
                    return False
                if board[y + py][x + px] == player.value:
                    neighbor_cnt += 1
                board[y + py][x + px] = -player.value
        if neighbor_cnt == 1:
            self.board = board
            return True
        return False

    def set_start_position(self, y=None, x=None):
        if not y or not x:
            y = randint(0, self.size - 1)
            x = randint(0, self.size - 1)
        self.board[y][x] = 1
        y = -y if y else self.size - 1
        x = -x if x else self.size - 1
        self.board[y][x] = 2
