from dataclasses import dataclass
from Address import Address
from Value import Value

@dataclass
class Store:
    s: dict[Address, Value]
    def __init__(self, s = {}):
        self.s = s
    
    def __str__(self):
        dictionary = {str(k) : str(v) for k,v in self.s.items()}
        return f"{dictionary}"