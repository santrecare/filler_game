from random import getrandbits, randint


class Piece:
    def __init__(self, size, density):
        self.size = size
        self.density = density
        self.piece = [[None for _ in range(0, self.size)] for _ in range(0, self.size)]
        self.generate_piece(x=randint(0, self.size - 1), y=randint(0, self.size - 1))

    def generate_piece(self, x, y, piece_size=0):
        piece_size += 1
        self.piece[y][x] = '*'
        if x + 1 < self.size and getrandbits(self.density) == 1 and not self.piece[y][x + 1]:
            piece_size = self.generate_piece(x + 1, y, piece_size)  # right
        if y + 1 < self.size and getrandbits(self.density) == 1 and not self.piece[y + 1][x]:
            piece_size = self.generate_piece(x, y + 1, piece_size)  # up
        if x - 1 >= 0 and getrandbits(self.density) == 1 and not self.piece[y][x - 1]:
            piece_size = self.generate_piece(x - 1, y, piece_size)  # left
        if y - 1 >= 0 and getrandbits(self.density) == 1 and not self.piece[y - 1][x]:
            piece_size = self.generate_piece(x, y - 1, piece_size)  # down
        if x + 1 < self.size and y + 1 < self.size and getrandbits(self.density) == 1\
                and not self.piece[y + 1][x + 1]:
            piece_size = self.generate_piece(x + 1, y + 1, piece_size)  # right + up
        if y + 1 < self.size and x - 1 >= 0 and getrandbits(self.density) == 1\
                and not self.piece[y + 1][x - 1]:
            piece_size = self.generate_piece(x - 1, y + 1, piece_size)  # left + up
        if x - 1 >= 0 and y - 1 >= 0 and getrandbits(self.density) == 1 and not self.piece[y - 1][x - 1]:
            piece_size = self.generate_piece(x - 1, y  - 1, piece_size)  # left + down
        if y - 1 >= 0 and x + 1 < self.size and getrandbits(self.density) == 1\
                and not self.piece[y - 1][x + 1]:
            piece_size = self.generate_piece(x + 1, y - 1, piece_size)  # right + down
        if piece_size < 2:
            piece_size = self.generate_piece(x, y, piece_size - 1)
        return piece_size
