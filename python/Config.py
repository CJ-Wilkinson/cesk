from python.cesk.Control import Control
from python.cesk.Kontinuation import Kontinuation
from python.cesk.Environment import Environment
from python.cesk.Store import Store
from dataclasses import dataclass
@dataclass
class Config:
    c: Control
    e: Environment
    s: Store
    k: Kontinuation

    def __str__(self):
        return f"<{self.c}, {self.e}, {self.s}, {self.k}>"