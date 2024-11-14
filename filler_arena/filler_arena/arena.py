class Arena:
    def __init__(self, board, first_player, second_player):
        self.board = board
        self.board.set_start_position()
        self.piece = None
        self.first_player = first_player
        self.second_player = second_player
        self.turn = True

    def is_player_alive(self):
        return (self.first_player.alive or self.second_player.alive)

    def __get_player_turn(self):
        if self.first_player.alive and self.turn:
            self.turn = not self.turn if self.second_player.alive else self.turn
            return self.first_player
        elif self.second_player.alive and not self.turn:
            self.turn = not self.turn if self.first_player.alive else self.turn
            return self.second_player

    def play(self):
        player = self.__get_player_turn()
        if not player:
            return
        y, x = player.play(self.piece, self.board)
        if x is not None and y is not None \
                and self.board.insert_piece(self.piece, y, x, player):
            player.score += 1
        else:
            player.alive = False

    def set_piece(self, piece):
        self.piece = piece

    def get_arena_state(self):
        return {
            'board': self.board.board,
            'piece': self.piece.piece,
            'player1': self.first_player.get_infos(),
            'player2': self.second_player.get_infos(),
        }
