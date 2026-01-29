use chumsky::prelude::Parser;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Name<'src> {
    name: &'src str,
}

/// The control can be either an expression or a statement
#[derive(Clone)]
enum Control<'src> {
    E(Expr<'src>),
    S(Stmt<'src>),
}

#[derive(Debug, Clone)]
enum Value {
    IntV(i32),
    BoolV(bool),
    VoidV,
    ArrayV(Vec<Value>),
}

/// # Expressions
/// e := i32 | - (negative) | + | * | - (subtraction) | / | % | == | != | < |  <= | >= | label
///     | fn call | []
#[derive(Debug, Clone)]
enum Expr<'src> {
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
enum Type {
    IntT,
    BoolT,
    VoidT,
    ArrayT(Box<Type>),
}

/// # Statements
/// s := if | = | expression | declaration (e.g. `int x = 1;`) | return (e)? | {} | while | break
///     | continue

#[derive(Debug, Clone)]
enum Stmt<'src> {
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

/// # Function
/// a function consists of a name, a list of args, and a body statement
struct Fun<'src> {
    name: Name<'src>,
    args: Vec<(Type, Name<'src>)>,
    body: Stmt<'src>,
}

struct Program<'src> {
    funs: Vec<Fun<'src>>,
}

pub fn fun_lookup(name: &str) -> Fun {
    todo!()
}

// pub fn successor_lookup<'src>(stmt: &'src Stmt) -> &'src Stmt<'src> {
pub fn successor_lookup<'src>(stmt: &'src Control) -> &'src Control<'src> {
    todo!()
}

pub fn alloc() -> Address {
    todo!()
}

struct Address;

enum Kont<'src> {
    DeclK(Type, &'src str, &'src Stmt<'src>, &'src Kont<'src>),
}

struct Configuration<'src> {
    c: &'src Control<'src>,
    e: HashMap<&'src str, &'src Address>,
    s: HashMap<&'src Address, &'src Value>,
    k: &'src Kont<'src>,
}


impl<'src> Configuration<'src> {
    pub fn next(&self) -> Self {
        match self.c {
            Control::S(Stmt::If(condition, t, f)) => todo!(),
            Control::S(Stmt::Assign(l, r)) => todo!(),
            Control::S(Stmt::ExprStmt(expr)) => todo!(),
            Control::S(Stmt::Decl(typ, name, None)) => Self {
                c: successor_lookup(self.c), // &Control::S(successor_lookup(stmt)),
                e: self.e.clone(),
                s: self.s.clone(),
                k: self.k,
            },
            Control::S(stmt @ Stmt::Decl(typ, name, Some(init))) => {
                todo!()
                // Self {
                // 	c: Control::E(init),
                // 	e: self.e,
                // 	s: self.s,
                // 	k: Kont::DeclK(typ, name.name, successor_lookup(stmt), self.k),
                // }
                // let addr = alloc();
                // Self {
                // 	c: successor_lookup(self.c),
                // 	e: self.e.clone().insert(name.name, addr),
                // 	s: self.s.clone().insert(addr, init),
                // 	k: self.k,
                // }
            }
            Control::S(Stmt::Return(Some(expr))) => todo!(),
            Control::S(Stmt::Return(None)) => todo!(),
            Control::S(Stmt::Block(stmts)) => todo!(),
            Control::S(Stmt::While(condition, stmt)) => todo!(),
            Control::S(Stmt::Break) => todo!(),
            Control::S(Stmt::Continue) => todo!(),

            Control::E(e) => match e {
                Expr::Val(expr) => {
                    todo!()
                }
                Expr::Neg(expr) => todo!(),
                Expr::Add(expr, y) => todo!(),
                Expr::Mult(expr, y) => todo!(),
                Expr::Sub(expr, y) => todo!(),
                Expr::Div(expr, y) => todo!(),
                Expr::Rem(expr, y) => todo!(),
                Expr::Eq(expr, y) => todo!(),
                Expr::Neq(expr, y) => todo!(),
                Expr::Lt(expr, y) => todo!(),
                Expr::Gt(expr, y) => todo!(),
                Expr::Lte(expr, y) => todo!(),
                Expr::Gte(expr, y) => todo!(),
                Expr::Var(name) => todo!(),
                Expr::Call(name, exprs) => todo!(),
                Expr::Array(exprs) => todo!(),
            },
        }
    }
}

/*
   pub fn parser<'src>() -> impl Parser<'src, '&src str, Program<'src> {
   todo!()
   }
   */

pub fn parse(filename: &str) {
    let src = std::fs::read_to_string(filename).unwrap();
    println!("{src}");
    let stmts: Vec<_> = src.split_whitespace().collect();
    println!("stmts: {:?}", stmts);
}
