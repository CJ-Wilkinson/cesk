use crate::ast::{Expr, Fun, Name, Stmt, StmtContents, Type, Value};
use std::collections::HashMap;
use std::rc::Rc;

// TODO see if this can be turned back into &str
type Env<'tree> = HashMap<&'tree str, i32>;
type Store<'tree> = HashMap<i32, &'tree Value>;

/// The control can be either an expression or a statement
#[derive(Debug, Clone)]
pub enum Control<'tree> {
    // For external AST references
    AstExpr(&'tree Expr),
    // For external AST references
    AstStmt(&'tree Stmt<'tree>),
    // For evaluated expressions, move ownership into Control
    Expr(Expr),
    // need this?
    Stmt(Stmt<'tree>),
}

pub fn fun_lookup<'tree>(_name: &'tree str) -> Fun<'tree> {
    todo!()
}

// pub fn successor_lookup<'tree>(stmt: &'tree Stmt<'tree>) -> &'tree Stmt<'tree> {
pub fn successor_lookup<'tree>(_stmt: &'tree Stmt<'tree>) -> &'tree Stmt<'tree> {
    // todo!()
    _stmt
}

pub fn alloc() -> i32 {
    0
}

//#[derive(Debug, Clone, PartialEq, Eq, Hash)]
//struct Address;

#[derive(Debug)]
pub enum Kont<'tree> {
    Mt,
    // Kont for Declaration
    DeclK(
        // Environment for Kont
        Rc<Env<'tree>>,
        Type,
        Name,
        // Control for Kont
        Control<'tree>,
        // Nested Kont
        Rc<Kont<'tree>>,
    ),
    IfK(
        // Environment
        Rc<Env<'tree>>,
        // true branch
        Control<'tree>,
        // false branch
        Option<Control<'tree>>,
        // Alternate version from Stmt::If->False branch
        // Option<Box<Stmt>>
        // Kont
        Rc<Kont<'tree>>,
    ),
}

#[derive(Debug)]
struct Configuration<'tree> {
    c: Control<'tree>,
    e: Rc<Env<'tree>>,
    s: Rc<Store<'tree>>,
    k: Rc<Kont<'tree>>,
}

impl<'tree> Configuration<'tree> {
    pub fn next(&'tree self) -> Self {
        match self.c {
            Control::AstStmt(s) => {
                match &s.contents {
                    StmtContents::If(condition, true_branch, false_branch) => {
                        Self {
                            c: Control::AstExpr(&condition),
                            e: self.e.clone(),
                            s: self.s.clone(),
                            k: Rc::new(Kont::IfK(
                                self.e.clone(),
                                Control::AstStmt(&true_branch),
                                //&Control::AstStmt(false_branch.unwrap().as_ref().unwrap()),
                                match false_branch {
                                    Some(x) => Some(Control::AstStmt(x.as_ref())),
                                    None => None,
                                },
                                self.k.clone(),
                            )),
                        }
                    }
                    StmtContents::Assign(_l, _r) => todo!(),
                    StmtContents::ExprStmt(_expr) => todo!(),
                    StmtContents::Decl(_typ, _name, None) => Self {
                        c: Control::AstStmt(successor_lookup(&s)),
                        e: self.e.clone(),
                        s: self.s.clone(),
                        k: self.k.clone(),
                    },
                    StmtContents::Decl(typ, name, Some(init)) => Self {
                        c: Control::AstExpr(&init),
                        e: self.e.clone(),
                        s: self.s.clone(),
                        k: Rc::new(Kont::DeclK(
                            self.e.clone(),
                            typ.clone(),
                            name.clone(),
                            Control::AstStmt(successor_lookup(&s)),
                            self.k.clone(),
                        )),
                    },
                    StmtContents::Return(Some(_expr)) => todo!(),
                    StmtContents::Return(None) => todo!(),
                    StmtContents::Block(_stmts) => todo!(),
                    StmtContents::While(_condition, _stmt) => todo!(),
                    StmtContents::Break => todo!(),
                    StmtContents::Continue => todo!(),
                }
            }

            Control::AstExpr(e) => match e {
                // only spot to invoke kont
                Expr::Val(v) => self.invoke_kont(&v),
                Expr::Neg(_expr) => todo!(),
                // TODO: Add nested expressions
                Expr::ArithBinop(left, op, right) => {
                    // TODO let's clean this up a bit
                    let l = match left.as_ref() {
                        Expr::Val(Value::IntV(x)) => *x,
                        _ => panic!("left of binop"),
                    };
                    let r = match right.as_ref() {
                        Expr::Val(Value::IntV(x)) => *x,
                        _ => panic!("right of binop"),
                    };
                    Self {
                        c: Control::Expr(Expr::Val(Value::IntV(op.call(l, r)))),
                        e: self.e.clone(),
                        s: self.s.clone(),
                        k: self.k.clone(),
                    }
                }
                Expr::CompareBinop(_left, _op, _right) => todo!(),
                Expr::Var(_name) => todo!(),
                Expr::Call(_name, _exprs) => todo!(),
                Expr::Array(_exprs) => todo!(),
            },
            _ => todo!(),
        }
    }

    fn invoke_kont(&'tree self, v: &'tree Value) -> Self {
        match self.k.as_ref() {
            Kont::DeclK(e, _t, Name(n), succ, k) => {
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
            }
            _ => todo!(),
        }
    }
}

fn it_works() {
    let ast = Stmt::bare_stmt(StmtContents::Decl(
        Type::IntT,
        Name("CESK".to_string()),
        Some(Expr::Val(Value::IntV(42))),
    ));
    let sigma_0 = Configuration {
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

pub fn parse(_filename: &str) {
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
        let ast = Stmt::bare_stmt(StmtContents::Decl(
            Type::IntT,
            Name("CESK".to_string()),
            Some(Expr::Val(Value::IntV(42))),
        ));
        let sigma_0 = Configuration {
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
