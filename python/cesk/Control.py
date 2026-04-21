from dataclasses import dataclass
from typing import Literal, Optional

from cesk.Address import Address
from cesk.AstNode import Expr, Stmt, AstNode


@dataclass
class Control:
    node: AstNode

    def __str__(self):
        return f"{self.node}"
