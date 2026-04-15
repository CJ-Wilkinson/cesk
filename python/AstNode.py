from Value import Value


class AstNode:
    def __str__(self):
        return "error"


class Expr(AstNode):
    expr: str
    def __init__(self, expr = "expr"):
        self.expr = expr
    def __str__(self):
        return self.expr


class Stmt(AstNode):
    def __str__(self):
        return "stmt"


class Assign(Stmt):
    lval: str
    expr: str
    def __init__(self, lval="lval", expr="expr"):
        self.lval = lval
        self.expr = expr
        
    def __str__(self):
        return f"Assign({self.lval}, {self.expr})"
