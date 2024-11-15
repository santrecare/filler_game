# PHP player template

```
{
    "piece_infos": {
        "witdh": int,
        "height": int,
        "density": float,
        "is_horizontal": bool,
        "is_vertical": bool,
    },
    "playable_coordinates": [{
        "coordinates": [int, int],
        "distance": float,
        "contact_zone_size": float,
        "top_border": bool,
        "bottom_border": bool,
        "left_border": bool,
        "right_border": bool,
    }, { ... }],
    "board_infos": {
        "fight_started": bool,
        "player": {
            "north": {
                "percent": int,
                "top_border": bool,
                "bottom_border": bool,
                "left_border": bool,
                "right_border": bool,
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
        "opponent": { ... }
    }
}
```
