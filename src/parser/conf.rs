use chumsky::prelude::Parser;
use std::collections::HashMap;
use std::rc::Rc;

type Env<'src> = HashMap<&'src str, i32>;
type Store<'src> = HashMap<i32, &'src Value>;

// Why though? Just &str might work?
#[derive(Debug, Clone)]
struct Name<'src> {
    name: &'src str,
}

/// The control can be either an expression or a statement
#[derive(Debug, Clone)]
enum Control<'src> {
    // For external AST references
    AstExpr(&'src Expr<'src>),
    // For external AST references
    AstStmt(&'src Stmt<'src>),
    // For evaluated expressions, move ownership into Control
    Expr(Expr<'src>),
    // need this?
    Stmt(Stmt<'src>),
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
    Decl(Type, Box<Name<'src>>, Option<Box<Expr<'src>>>),
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
pub fn successor_lookup<'src>(stmt: &'src Stmt<'src>) -> &'src Stmt<'src> {
    &Stmt::Break
}

pub fn alloc() -> i32 {
    0
}

//#[derive(Debug, Clone, PartialEq, Eq, Hash)]
//struct Address;

#[derive(Debug)]
enum Kont<'src> {
	Mt,
	// Kont for Declaration
    DeclK(
    	// Environment for Kont
        Rc<Env<'src>>,
        Type,
        &'src str,
        // Control for Kont
        Control<'src>,
        // Nested Kont
        Rc<Kont<'src>>,
    ),
    IfK(
        // Environment
        Rc<Env<'src>>,
        // true branch
        Control<'src>,
        // false branch
        Option<Control<'src>>,
        // Alternate version from Stmt::If->False branch
        // Option<Box<Stmt<'src>>>
        // Kont
        Rc<Kont<'src>>,
    ),
}

#[derive(Debug)]
struct Configuration<'src> {
    c: Control<'src>,
    e: Rc<Env<'src>>,
    s: Rc<Store<'src>>,
    k: Rc<Kont<'src>>,
}

impl<'src> Configuration<'src> {
    pub fn next(&self) -> Self {
        match self.c {
            Control::AstStmt(Stmt::If(condition, true_branch, false_branch)) => {
                Self {
                    c: Control::AstExpr(condition),
                    e: self.e.clone(),
                    s: self.s.clone(),
                    k: Rc::new(Kont::IfK(
                        self.e.clone(),
                        Control::AstStmt(true_branch),
                        //&Control::AstStmt(false_branch.unwrap().as_ref().unwrap()),
                        match false_branch {
                            Some(x) => Some(Control::AstStmt(x.as_ref())),
                            None => None,
                        },
                        self.k.clone(),
                    )),
                }
            },
            Control::AstStmt(Stmt::Assign(l, r)) => todo!(),
            Control::AstStmt(Stmt::ExprStmt(expr)) => todo!(),
            Control::AstStmt(stmt @ Stmt::Decl(typ, name, None)) => Self {
                c: Control::AstStmt(successor_lookup(stmt)),
                e: self.e.clone(),
                s: self.s.clone(),
                k: self.k.clone(),
            },
            Control::AstStmt(stmt @ Stmt::Decl(typ, name, Some(init))) => Self {
                c: Control::AstExpr(init),
                e: self.e.clone(),
                s: self.s.clone(),
                k: Rc::new(Kont::DeclK(
                    self.e.clone(),
                    typ.clone(),
                    name.name,
                    Control::AstStmt(successor_lookup(stmt)),
                    self.k.clone(),
                )),
            },
            Control::AstStmt(Stmt::Return(Some(expr))) => todo!(),
            Control::AstStmt(Stmt::Return(None)) => todo!(),
            Control::AstStmt(Stmt::Block(stmts)) => todo!(),
            Control::AstStmt(Stmt::While(condition, stmt)) => todo!(),
            Control::AstStmt(Stmt::Break) => todo!(),
            Control::AstStmt(Stmt::Continue) => todo!(),

            Control::AstExpr(e) => match e {
                // only spot to invoke kont
                Expr::Val(v) => self.invoke_kont(&v),
                Expr::Neg(expr) => todo!(),
                // TODO: Add nested expressions
                Expr::Add(left, right) => {
                    let l = match left.as_ref() {
                        Expr::Val(Value::IntV(x)) => x,
                        _ => panic!("left of add"),
                    };
                    let r = match right.as_ref() {
                        Expr::Val(Value::IntV(x)) => x,
                        _ => panic!("right of add"),
                    };
                    Self {
                      c: Control::Expr(Expr::Val(Value::IntV(l + r))),
                      e: self.e.clone(),
                      s: self.s.clone(),
                      k: self.k.clone(),
                    }
                },
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
            _ => todo!(),
        }
    }

    fn invoke_kont(&self, v: &'src Value) -> Self {
        match self.k.as_ref() {
            Kont::DeclK(e, t, n, succ, k) => {
                let addr = alloc();
                let mut e_prime: Env = (**e).clone();
                e_prime.insert(n, addr);
                let mut s_prime = (*self.s).clone();
                s_prime.insert(addr, v);
                Self {
                    c: succ.clone(),
                    e: Rc::new(e_prime),
                    s: Rc::new(s_prime),
                    k: Rc::clone(k),
                }
            },
            _ => todo!(),
        }
    }
}

/*
pub fn parser<'src>() -> impl Parser<'src, '&src str, Program<'src> {
todo!()
}
*/

    fn it_works() {
        let ast = Stmt::Decl(
            Type::IntT,
            Box::new(Name{
                name: "CESK",
            }),
            Some(Box::new(Expr::Val(Value::IntV(42)))),
        );
        let sigma_0 = Configuration{
            c: Control::AstStmt(&ast),
            e: Rc::new(HashMap::new()),
            s: Rc::new(HashMap::new()),
            k: Rc::new(Kont::Mt),
        };
        let sigma_1 = sigma_0.next();
        println!("{:?}", sigma_1);
        let sigma_2 = sigma_1.next();
        println!("{:?}", sigma_2);
    }

pub fn parse(filename: &str) {
    println!("Hello");
    it_works();
    //let src = std::fs::read_to_string(filename).unwrap();
    //println!("{src}");
    //let stmts: Vec<_> = src.split_whitespace().collect();
    //println!("stmts: {:?}", stmts);
}

// src/lib.rs or src/main.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ast = Stmt::Decl(
            Type::IntT,
            Box::new(Name{
                name: "CESK",
            }),
            Some(Box::new(Expr::Val(Value::IntV(42)))),
        );
        let sigma_0 = Configuration{
            c: Control::AstStmt(&ast),
            e: Rc::new(HashMap::new()),
            s: Rc::new(HashMap::new()),
            k: Rc::new(Kont::Mt),
        };
        let sigma_1 = sigma_0.next();
        println!("{:?}", sigma_1);
        let sigma_1 = sigma_0.next();
        println!("{:?}", sigma_1);
    }
}
