// ewwwwww
use chumsky::prelude::Parser;

#[derive(Debug)]
struct Name<'src> {
    name: &'src str,
}

#[derive(Debug)]
enum Expr<'src> {
    Num(i32),

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
    ArrayExpr(Vec<Expr<'src>>),
}

#[derive(Debug)]
enum Type {
    Int,
    Bool,
    Void,
    Array(Box<Type>),
}

#[derive(Debug)]
enum Stmt<'src> {
    If(Box<Expr<'src>>, Box<Stmt<'src>>, Option<Box<Stmt<'src>>>),
    Assign(Box<Expr<'src>>, Box<Expr<'src>>),
    ExprStmt(Box<Expr<'src>>),
    Decl(Box<Type>, Box<Name<'src>>, Box<Expr<'src>>),
    Return(Option<Box<Expr<'src>>>),
    Block(Vec<Stmt<'src>>),
    While(Box<Expr<'src>>, Box<Stmt<'src>>),
    Break,
    Continue,
}

struct Fun<'src> {
    name: Name<'src>,
    args: Vec<(Type, Name<'src>)>,
    body: Stmt<'src>,
}

struct Program<'src> {
    funs: Vec<Fun<'src>>,
}

/*
pub fn parser<'src>() -> impl Parser<'src, '&src str, Program<'src> {
    todo!()
}
*/

pub fn parse(filename: &str) {
    let src = std::fs::read_to_string(filename).unwrap();
}
