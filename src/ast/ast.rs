#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    IntV(i32),
    BoolV(bool),
    VoidV,
    ArrayV(Vec<Value>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithBinop {
    Add,
    Mult,
    Sub,
    Div,
    Rem,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompareBinop {
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
}

impl ArithBinop {
    pub fn call(&self, lhs: i32, rhs: i32) -> i32 {
        match self {
            Self::Add => lhs + rhs,
            Self::Sub => lhs - rhs,
            Self::Mult => lhs * rhs,
            Self::Div => lhs / rhs,
            Self::Rem => lhs % rhs,
        }
    }
}

impl CompareBinop {
    pub fn call(&self, lhs: i32, rhs: i32) -> bool {
        match self {
            Self::Eq => lhs == rhs,
            Self::Neq => lhs != rhs,
            Self::Lt => lhs < rhs,
            Self::Gt => lhs > rhs,
            Self::Lte => lhs <= rhs,
            Self::Gte => lhs >= rhs,
        }
    }
}

/// # Expressions
/// e := i32 | - (negative) | + | * | - (subtraction) | / | % | == | != | < |  <= | >= | label
///     | fn call | []
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Val(Value),

    Neg(Box<Expr>),
    ArithBinop(Box<Expr>, ArithBinop, Box<Expr>),
    CompareBinop(Box<Expr>, CompareBinop, Box<Expr>),

    Var(Name),

    Call(Name, Vec<Expr>),
    Array(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Type {
    IntT,
    BoolT,
    VoidT,
    ArrayT(Box<Type>),
}

/// # Statements
/// s := if | = | expression | declaration (e.g. `int x = 1;`) | return (e)? | {} | while | break
///     | continue

#[derive(Debug, Clone)]
pub enum Stmt {
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Assign(Expr, Expr),
    ExprStmt(Expr),
    Decl(Type, Name, Option<Expr>),
    Return(Option<Expr>),
    Block(Vec<Stmt>),
    While(Expr, Box<Stmt>),
    Break,
    Continue,
}

/// # Function
/// a function consists of a return type, a name, a list of args, and a body statement
#[derive(Debug, Clone)]
pub struct Fun {
    pub rtype: Type,
    pub name: Name,
    pub args: Vec<(Type, Name)>,
    pub body: Stmt,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub funs: Vec<Fun>,
}
