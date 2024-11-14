from collections import defaultdict
from random import randint

from .distance import distance


class Board:
    def __init__(self, board):
        self.size = len(board)
        self.board = board
        self.piece = None

    def __is_valid_box(self, y, x, player):
        return (x >= 0 and x < self.size and y >= 0 and y < self.size
            and (not self.board[y][x] or self.board[y][x] == player))

    def is_valid_piece_coord(self, piece, y, x):
        neighbor_cnt = 0
        for py in range(0, piece.size):
            for px in range(0, piece.size):
                if not piece.piece[py][px]:
                    continue
                if not self.__is_valid_box(py + y, px + x, piece.player):
                    return False
                if self.board[y + py][x + px] == piece.player:
                    neighbor_cnt += 1
        if neighbor_cnt == 1:
            return True
        return False

    # def __is_piece_insertable(self, x, y):
    #     if x < 0 or x >= self.size or y < 0 or y >= self.size or self.board[y][x]:
    #         return False
    #     return True
    #
    # def __is_piece_neighbor(self, x, y, player):
    #     if (x + 1 < self.size and self.board[y][x + 1] == player)\
    #             or (y + 1 < self.size and self.board[y + 1][x] == player)\
    #             or (x - 1 >= 0 and self.board[y][x - 1] == player)\
    #             or (y - 1 >= 0 and self.board[y - 1][x] == player):
    #         return True
    #     return False
    #
    # def is_valid_piece_coord(self, piece, y, x):
    #     is_neighbor = False
    #     for py in range(0, piece.size):
    #         for px in range(0, piece.size):
    #             if not piece.piece[py][px]:
    #                 continue
    #             if not self.__is_piece_insertable(px + x, py + y):
    #                 return False
    #             if not is_neighbor:
    #                 is_neighbor = self.__is_piece_neighbor(
    #                     px + x, py + y, piece.player)
    #     return is_neighbor

    def set_piece(self, piece, y, x):
        piece.coords = []
        is_neighbor = False
        for py in range(0, piece.size):
            for px in range(0, piece.size):
                if not piece.piece[py][px]:
                    continue
                self.board[y + py][x + px] = -piece.player
                piece.coords.append((y + py, x + px))
        self.piece = piece

    def __compute_box_influencers(self, zone_range, by, bx):
        influencers = set()
        point = 0
        for zy in zone_range:
            for zx in zone_range:
                x = bx + zx
                y = by + zy
                point += 1
                if x < self.size and x >= 0 and y < self.size and y >= 0 and self.board[y][x]:
                    influencers.add(abs(self.board[y][x]))
                    point -= 1
        return influencers, point

    def compute_players_influence(self, zone_size):
        players_influence = defaultdict(int)
        zone_range = range(-zone_size, zone_size + 1)
        for by in range(0, self.size):
            for bx in range(0, self.size):
                if self.board[by][bx]:
                    continue
                box_influencers, point = self.__compute_box_influencers(zone_range, by, bx)
                if len(box_influencers) == 1:
                    players_influence[next(iter(box_influencers))] += point
        return players_influence

    def compute_piece_distance(self):
        piece_parts_distance = []
        for coord in self.piece.coords:
            r = 1
            shortest_dist = distance((0, 0), (self.size, self.size))
            while shortest_dist > r and r <= self.size:
                for y in range(-r, r + 1):
                    for x in range(-r, r + 1):
                        if y + coord[0] >= 0 and y + coord[0] < self.size and\
                            x + coord[1] >= 0 and x + coord[1] < self.size and\
                            (((y != -r or y != r) and (x == -r or x == r))\
                            or y == -r or y == r) and\
                            self.board[coord[0] + y][coord[1] + x] and\
                            abs(self.board[coord[0] + y][coord[1] + x]) != self.piece.player:
                                shortest_dist = distance(
                                    (coord[0] + y, coord[1] + x),
                                    (coord[0], coord[1])
                                )
                r += 1
            piece_parts_distance.append(shortest_dist)
        return piece_parts_distance

    # def eval(self):
    #     score = 0
    #     player_score = 0
    #     for key, val in self.compute_players_influence(self.piece.size).items():
    #         if key == self.piece.player:
    #             player_score = val
    #         else:
    #             score = val
    #     player_score += player_score - score
    #     return player_score

    def eval(self):
        dist = self.compute_piece_distance()
        return -(sum(dist) / len(dist))
