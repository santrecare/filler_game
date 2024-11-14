from dataclasses import dataclass, field
from typing import List, Tuple


@dataclass
class Piece:
    piece: List[List[str]]
    size: int
    player: int
    coords: List[Tuple[int, int]] = field(default_factory=list)
