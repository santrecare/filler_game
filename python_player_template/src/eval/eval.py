def eval(game_infos, board, piece, player_value):
    """
    parameters
    ----------
    gameInfos: {
        "piece_infos": {
            "witdh": int,           // Real width of the piece
            "height": int,          // Real height of the piece
            "density": float,       // Density of the piece -> piece_cells * 100 / witdh * height
            "is_horizontal": bool,  // Piece orientation -> witdh >= height
            "is_vertical": bool,    // Piece orientation -> height >= witdh
        },
        "playable_coordinates": [{
            "coordinates": [int, int],  // [y, x]
            "distance": float,          // Sum of the shortest distance between each piece cells with opponent
            "contact_zone_size": int,   // Sum of opponent cells in contact with each cell of the piece placed
            "top_border": bool,         // Touch or not the board border
            "bottom_border": bool,      // Touch or not the board border
            "left_border": bool,        // Touch or not the board border
            "right_border": bool,       // Touch or not the board border
        }, { ... }],
        "board_infos": {
            "fight_started": bool, // Check if the players met
            "player": { 
                "north": {
                    "percent": float,      // Percentage of occupation of northern part of the board
                    "top_border": bool,    // Touch or not the upper edge of the northern part of the board
                    "bottom_border": bool, // Touch or not the lower edge of the northern part of the board
                    "left_border": bool,   // Touch or not the left edge of the northern part of the board
                    "right_border": bool,  // Touch or not the right edge of the northern part of the board
                },
                "south": { ... },
                "east": { ... },
                "west": { ... },
                "north_east": { ... },
                "north_west": { ... },
                "south_east": { ... },
                "south_west": { ... },
                "center": { ... }
            },
            "opponent": { ... } // Same information as the player but for the opponent
        }
    }
        All infos from the game
    board: list[list[int]]
        Current board state
    piece: list[list[char]]
        Piece to place on the board.
    playerValue: int
        Player value on the board, 1 or 2.
    
    return
    ------
    list[int, int] || None
         Coordinates to play.
    """
    move = None
    distance = float('inf')
    for coord in game_infos['playable_coordinates']:
        if distance <= coord['distance']:
            continue
        distance = coord['distance']
        move = coord['coordinates']
    return move
