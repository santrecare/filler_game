from random import getrandbits, randint


class Piece:
    def __init__(self, size):
        self.size = size
        self.piece = [[None for _ in range(0, self.size)] for _ in range(0, self.size)]
        self.generate_piece(x=randint(0, self.size - 1), y=randint(0, self.size - 1))

    def generate_piece(self, x, y):
        self.piece[y][x] = '*'
        if x + 1 < self.size and getrandbits(2) == 1 and not self.piece[y][x + 1]:
            self.generate_piece(x + 1, y)  # right
        if y + 1 < self.size and getrandbits(2) == 1 and not self.piece[y + 1][x]:
            self.generate_piece(x, y + 1)  # up
        if x - 1 >= 0 and getrandbits(2) == 1 and not self.piece[y][x - 1]:
            self.generate_piece(x - 1, y)  # left
        if y - 1 >= 0 and getrandbits(2) == 1 and not self.piece[y - 1][x]:
            self.generate_piece(x, y - 1)  # down
        if x + 1 < self.size and y + 1 < self.size and getrandbits(2) == 1\
                and not self.piece[y + 1][x + 1]:
            self.generate_piece(x + 1, y + 1)  # right + up
        if y + 1 < self.size and x - 1 >= 0 and getrandbits(2) == 1\
                and not self.piece[y + 1][x - 1]:
            self.generate_piece(x - 1, y + 1)  # left + up
        if x - 1 >= 0 and y - 1 >= 0 and getrandbits(2) == 1 and not self.piece[y - 1][x - 1]:
            self.generate_piece(x - 1, y  - 1)  # left + down
        if y - 1 >= 0 and x + 1 < self.size and getrandbits(2) == 1\
                and not self.piece[y - 1][x + 1]:
            self.generate_piece(x + 1, y - 1)  # right + down

    def description(self):
        piece = ''
        for y in range(0, self.size):
            for x in range(0, self.size):
                piece += f'{"*" if self.piece[y][x] else "."}'
            piece += '\n'
        print(f'size: {self.size}\npiece:\n{piece}')
