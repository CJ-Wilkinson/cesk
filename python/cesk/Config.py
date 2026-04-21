from cesk.Control import Control
from cesk.Kontinuation import Kontinuation
from cesk.Environment import Environment
from cesk.Store import Store
from dataclasses import dataclass
@dataclass
class Config:
    c: Control
    k: Kontinuation

    def __str__(self):
        return f"<{self.c}, {self.k}>"