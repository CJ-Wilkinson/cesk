from cesk.AstNode import Expr, Stmt
from dataclasses import dataclass, field
from functools import partial
from cesk.Value import Value

class Kontinuation:
    def __str__(self):
        return f"k"

@dataclass
class AssignK(Kontinuation):
    lval: Expr = field(default_factory=partial(Expr, "lval"))
    succ: Stmt = field(default_factory=Stmt)

    def __str__(self):
        return f"AssignK({self.lval}, {self.succ})"


@dataclass
class LvalK(Kontinuation):
    val: Value = field(default_factory=partial(Value, "val"))
    succ: Stmt = field(default_factory=Stmt)

    def __str__(self):
        return f"LvalK({self.val}, {self.succ})"
