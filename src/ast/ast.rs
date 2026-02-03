#[derive(Debug, Clone)]
pub struct Name<'src> {
    pub name: &'src str,
}

#[derive(Debug, Clone)]
pub enum Value {
    IntV(i32),
    BoolV(bool),
    VoidV,
    ArrayV(Vec<Value>),
}

/// # Expressions
/// e := i32 | - (negative) | + | * | - (subtraction) | / | % | == | != | < |  <= | >= | label
///     | fn call | []
#[derive(Debug, Clone)]
pub enum Expr<'src> {
    Val(Value),

    Neg(Box<Expr<'src>>),
    Add(Box<Expr<'src>>, Box<Expr<'src>>),
    Mult(Box<Expr<'src>>, Box<Expr<'src>>),
    Sub(Box<Expr<'src>>, Box<Expr<'src>>),
    Div(Box<Expr<'src>>, Box<Expr<'src>>),
    Rem(Box<Expr<'src>>, Box<Expr<'src>>),
    Eq(Box<Expr<'src>>, Box<Expr<'src>>),
    Neq(Box<Expr<'src>>, Box<Expr<'src>>),
    Lt(Box<Expr<'src>>, Box<Expr<'src>>),
    Gt(Box<Expr<'src>>, Box<Expr<'src>>),
    Lte(Box<Expr<'src>>, Box<Expr<'src>>),
    Gte(Box<Expr<'src>>, Box<Expr<'src>>),

    Var(Box<Name<'src>>),

    Call(Box<Name<'src>>, Vec<Expr<'src>>),
    Array(Vec<Expr<'src>>),
}

#[derive(Debug, Clone)]
pub enum Type {
    IntT,
    BoolT,
    VoidT,
    ArrayT(Box<Type>),
}

/// # Function
/// a function consists of a name, a list of args, and a body statement
pub struct Fun<'src> {
    pub name: Name<'src>,
    pub args: Vec<(Type, Name<'src>)>,
    pub body: Stmt<'src>,
}

pub struct Program<'src> {
    pub funs: Vec<Fun<'src>>,
}

/// # Statements
/// s := if | = | expression | declaration (e.g. `int x = 1;`) | return (e)? | {} | while | break
///     | continue

#[derive(Debug, Clone)]
pub enum Stmt<'src> {
    If(Box<Expr<'src>>, Box<Stmt<'src>>, Option<Box<Stmt<'src>>>),
    Assign(Box<Expr<'src>>, Box<Expr<'src>>),
    ExprStmt(Box<Expr<'src>>),
    Decl(Box<Type>, Box<Name<'src>>, Option<Box<Expr<'src>>>),
    Return(Option<Box<Expr<'src>>>),
    Block(Vec<Stmt<'src>>),
    While(Box<Expr<'src>>, Box<Stmt<'src>>),
    Break,
    Continue,
}
