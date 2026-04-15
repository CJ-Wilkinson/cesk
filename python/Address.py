

class Address:
    addr: int

    def __init__(self, addr):
        self.addr = addr

    def __str__(self):
        return f"{self.addr}"
    
    def __hash__(self):
        return hash(self.addr)