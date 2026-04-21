from dataclasses import dataclass
from cesk.Address import Address
import json

@dataclass
class Environment:
    e: dict[str, Address]
    def __init__(self, e={}):
        self.e = e

    def __str__(self):
        dictionary = {k : str(v) for k,v in self.e.items()}
        return f"{dictionary}"
        