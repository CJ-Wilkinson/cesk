use chumsky::prelude::Parser;
use std::collections::HashMap;

#[derive(Debug)]
struct Name<'src> {
    name: &'src str,
}

enum Control<'src> {
	E(Expr<'src>),
	S(Stmt<'src>),
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
    Array(Vec<Expr<'src>>),
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
    Decl(Box<Type>, Box<Name<'src>>, Option<Box<Expr<'src>>>),
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


pub fn fun_lookup(name: &str) -> Fun {
	todo!()
}

pub fn successor_lookup(stmt: &Stmt) -> &Stmt {
	todo!()
}

pub fn alloc() -> Address {
	todo!()
}

struct Address;

struct Value;

enum Kont<'src> {
	DeclK(Type, &'src str, &'src Stmt<'src>, &'src Kont),
}

struct Configuration<'src> {
	c: &'src Control<'src>,
	e: HashMap<&'src str, &'src Address>,
	s: HashMap<&'src Address, &'src Value>,
	k: &'src Kont,
}

impl<'src> Configuration<'src> {
	pub fn next(&self) -> Self {
		match self.c {
			Control::S(Stmt::If(condition, t, f)) => todo!(),
			Control::S(Stmt::Assign(l, r)) => todo!(),
			Control::S(Stmt::ExprStmt(expr)) => todo!(),
			Control::S(Stmt::Decl(typ, name, None)) => Self {
				c: successor_lookup(self.c),
				e: self.e.clone(),
				s: self.s,
				k: self.k,
			},
			Control::S(stmt @ Stmt::Decl(typ, name, Some(init))) => {
				Self {
					c: Control::E(init),
					e: self.e,
					s: self.s,
					k: Kont::DeclK(typ, name.name, successor_lookup(stmt), self.k),
				}
			// let addr = alloc();
			// Self {
			// 	c: successor_lookup(self.c),
			// 	e: self.e.clone().insert(name.name, addr),
			// 	s: self.s.clone().insert(addr, init),
			// 	k: self.k,
			// }
			},
			Control::S(Stmt::Return(Some(expr))) => todo!(),
			Control::S(Stmt::Return(None)) => todo!(),
			Control::S(Stmt::Block(stmts)) => todo!(),
			Control::S(Stmt::While(condition, stmt)) => todo!(),
			Control::S(Stmt::Break) => todo!(),
			Control::S(Stmt::Continue) => todo!(),
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
}
