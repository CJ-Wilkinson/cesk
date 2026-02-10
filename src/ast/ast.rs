use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    IntV(i32),
    BoolV(bool),
    VoidV,
    ArrayV(Vec<Value>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArithBinop {
    Add,
    Mult,
    Sub,
    Div,
    Rem,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompareBinop {
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Val(Value),

    Neg(Box<Expr>),
    ArithBinop(Box<Expr>, ArithBinop, Box<Expr>),
    CompareBinop(Box<Expr>, CompareBinop, Box<Expr>),

    Var(Name),

    Call(Name, Vec<Expr>),
    Array(Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    IntT,
    BoolT,
    VoidT,
    ArrayT(Box<Type>),
}

/// # Statements
/// s := if | = | expression | declaration (e.g. `int x = 1;`) | return (e)? | {} | while | break
///     | continue

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt<'tree> {
    If(Expr, Box<Stmt<'tree>>, Option<Box<Stmt<'tree>>>),
    Assign(Expr, Expr),
    ExprStmt(Expr),
    Decl(Type, Name, Option<Expr>),
    Return(Option<Expr>),
    Block(Vec<Stmt<'tree>>, HashMap<(), ()>),
    //Block(Vec<Stmt<'tree>>, HashMap<&'tree Stmt<'tree>, &'tree Stmt<'tree>>),
    While(Expr, Box<Stmt<'tree>>),
    Break,
    Continue,
}

impl<'tree> Stmt<'tree> {
    pub fn make_block(stmts: Vec<Stmt<'tree>>) -> Self {
        Self::Block(stmts, HashMap::new())
        //Self::Block(stmts, stmts.iter().zip(stmts.iter().skip(1)).collect())
    }
}

/// # Function
/// a function consists of a return type, a name, a list of args, and a body statement
#[derive(Debug, Clone)]
pub struct Fun<'tree> {
    pub rtype: Type,
    pub name: Name,
    pub args: Vec<(Type, Name)>,
    pub body: Stmt<'tree>,
}

#[derive(Debug, Clone)]
pub struct Program<'tree> {
    pub funs: Vec<Fun<'tree>>,
}
