from Control import Control
from Environment import Environment
from Store import Store
from Kontinuation import Kontinuation
from dataclasses import dataclass
@dataclass
class Config:
    c: Control
    e: Environment
    s: Store
    k: Kontinuation

    def __str__(self):
        return f"<{self.c}, {self.e}, {self.s}, {self.k}>"