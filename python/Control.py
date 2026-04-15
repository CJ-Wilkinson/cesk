from dataclasses import dataclass
from typing import Literal, Optional

from Address import Address
from AstNode import Expr, Stmt, AstNode


@dataclass
class Control:
    node: AstNode

    def __str__(self):
        return f"{self.node}"
