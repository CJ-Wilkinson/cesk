use crate::ast::{Expr, Fun, Name, Stmt, Type, Value, CompareBinop};
use std::collections::HashMap;
use std::rc::Rc;

// TODO see if this can be turned back into &str
type Env<'tree> = HashMap<&'tree str, i32>;
type Store = HashMap<i32, Rc<Value>>;

/// The control can be either an expression or a statement
#[derive(Debug, Clone)]
pub enum Control {
    // For external AST references
    AstExpr(Rc<Expr>),
    // For external AST references
    AstStmt(Rc<Stmt>),
    // For evaluated expressions, move ownership into Control
    Expr(Rc<Expr>),
    // need this?
    Stmt(Stmt),
}

pub fn fun_lookup<'tree>(_name: &'tree str) -> Fun {
    todo!()
}

// pub fn successor_lookup<'tree>(stmt: &'tree Stmt<'tree>) -> &'tree Stmt<'tree> {
pub fn successor_lookup<'tree>(_stmt: Rc<Stmt>) -> Rc<Stmt> {
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
        //Control<'tree>,
        Rc<Stmt>,
        // Nested Kont
        Rc<Kont<'tree>>,
    ),
    IfK(
        // Environment
        Rc<Env<'tree>>,
        // true branch
        Rc<Stmt>,
        // false branch
        //Option<Control<'tree>>,
        Option<Rc<Stmt>>,
        // Alternate version from Stmt::If->False branch
        // Option<Box<Stmt>>
        // Kont
        Rc<Kont<'tree>>,
    ),
    OpK(
    	CompareBinop,
    	Rc<Expr>,
    	Rc<Kont<'tree>>,
    ),
    ExprStmtK(Rc<Stmt>, Rc<Kont<'tree>>),
}

#[derive(Debug)]
struct Configuration<'tree> {
    c: Control,
    e: Rc<Env<'tree>>,
    s: Rc<Store>,
    k: Rc<Kont<'tree>>,
}

impl<'tree> Configuration<'tree> {
    pub fn next(&'tree self) -> Self {
        match self.c.clone() {
            // TODO Make this work without cloning it
            Control::AstStmt(s) => {
                match &*s {
                    Stmt::If(condition, true_branch, false_branch) => {
                        Self {
                            c: Control::AstExpr(Rc::clone(condition)),
                            e: self.e.clone(),
                            s: self.s.clone(),
                            k: Rc::new(Kont::IfK(
                                self.e.clone(),
                                Rc::clone(true_branch),
                                //&Control::AstStmt(false_branch.unwrap().as_ref().unwrap()),
                                match false_branch {
                                    Some(x) => Some(Rc::clone(x)),
                                    None => None,
                                },
                                self.k.clone(),
                            )),
                        }
                    }
                    Stmt::Assign(_l, _r) => todo!(),
                    // Stmt::ExprStmt(expr) => Self {
                    //     c: Control::AstExpr(expr),
                    //     e: Rc::clone(&self.e),
                    //     s: Rc::clone(&self.s),
                    //     k: Rc::new(Kont::ExprStmtK(successor_lookup(s), Rc::clone(&self.k))),
                    // },
                    Stmt::ExprStmt(_expr) => todo!(),
                    Stmt::Decl(_typ, _name, None) => Self {
                        c: Control::AstStmt(successor_lookup(s)),
                        e: self.e.clone(),
                        s: self.s.clone(),
                        k: self.k.clone(),
                    },
                    Stmt::Decl(typ, name, Some(init)) => Self {
                        c: Control::AstExpr(Rc::clone(init)),
                        e: self.e.clone(),
                        s: self.s.clone(),
                        k: Rc::new(Kont::DeclK(
                            self.e.clone(),
                            typ.clone(),
                            name.clone(),
                            successor_lookup(s),
                            self.k.clone(),
                        )),
                    },
                    Stmt::Return(Some(_expr)) => todo!(),
                    Stmt::Return(None) => todo!(),
                    // Return should insert a void into the returned value if None
                    Stmt::Block(_stmts) => todo!(),
                    //Stmt::While(_condition, _stmt) => todo!(),
                    Stmt::Break => todo!(),
                    Stmt::Continue => todo!(),
                    Stmt::Goto(_label) => todo!(),
                }
            },

            Control::AstExpr(e) => match &*e {
                // only spot to invoke kont
                Expr::Val(v) => self.invoke_kont(v),
                // Expr::Neg(_expr) => todo!(),
                // // TODO: Add nested expressions
                // Expr::ArithBinop(left, op, right) => {
                //     // TODO let's clean this up a bit
                //     let l = match left.as_ref() {
                //         Expr::Val(Value::IntV(x)) => *x,
                //         _ => panic!("left of binop"),
                //     };
                //     let r = match right.as_ref() {
                //         Expr::Val(Value::IntV(x)) => *x,
                //         _ => panic!("right of binop"),
                //     };
                //     Self {
                //         c: Control::Expr(Expr::Val(Value::IntV(op.call(l, r)))),
                //         e: self.e.clone(),
                //         s: self.s.clone(),
                //         k: self.k.clone(),
                //     }
                // }
                // Expr::CompareBinop(_left, _op, _right) => todo!(),
                // Expr::Var(_name) => todo!(),
                // Expr::Call(_name, _exprs) => todo!(),
                // Expr::Array(_exprs) => todo!(),
                // Expr::Index(_array, _index) => todo!(),
                //Expr::CompareBinop(Expr::Val(Value::IntV(left)), op, Expr::Val(Value::IntV(right))) => {
                	
                
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    fn invoke_kont(&'tree self, v: &Rc<Value>) -> Self {
        match self.k.as_ref() {
            Kont::DeclK(e, _t, Name(n), succ, k) => {
                let addr = alloc();
                let mut e_prime: Env = (**e).clone();
                e_prime.insert(n, addr);
                let mut s_prime = (*self.s).clone();
                s_prime.insert(addr, Rc::clone(v));
                Self {
                    c: Control::AstStmt(Rc::clone(succ)),
                    e: Rc::new(e_prime),
                    s: Rc::new(s_prime),
                    k: Rc::clone(k),
                }
            },
            Kont::IfK(_e, t, f, k) => match v.as_ref() {
                Value::BoolV(true) => Self {
                    c: Control::AstStmt(Rc::clone(t)),
                    e: Rc::clone(&self.e),
                    s: Rc::clone(&self.s),
                    k: Rc::clone(k),
                },
                Value::BoolV(false) => {
                    if let Some(fb) = f {
                        Self {
                            c: Control::AstStmt(Rc::clone(fb)),
                            e: Rc::clone(&self.e),
                            s: Rc::clone(&self.s),
                            k: Rc::clone(k),
                        }
                    } else {
                        todo!();
                    }
                },
                _ => todo!(),
            },
            // Kont::ExprStmtK(succ, k) => {
            // 	Self {
            // 		c: Control::AstStmt(Rc::clone(succ)),
            //         e: Rc::clone(&self.e),
            //         s: Rc::clone(&self.s),
            //         k: Rc::clone(k),
            // 	}
            // }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn decl_test() {
    // 	// Test ast
    //     let ast = Stmt::Decl(
    //         Type::IntT,
    //         Name("CESK".to_string()),
    //         Some(Expr::Val(Value::IntV(42))),
    //     );
    //     // Initial config
    //     let sigma_0 = Configuration {
    //         c: Control::AstStmt(&ast),
    //         e: Rc::new(HashMap::new()),
    //         s: Rc::new(HashMap::new()),
    //         k: Rc::new(Kont::Mt),
    //     };
    //     let sigma_1 = sigma_0.next();
    //     //println!("{:?}", sigma_1);
    //     let sigma_1 = sigma_0.next();
    //     //println!("{:?}", sigma_1);
    // }
    // #[test]
    // fn if_true_test() {
    //     // Test ast
    //     let ast = Rc::new(Stmt::If(
    //         Rc::new(Expr::Val(Rc::new(Value::BoolV(true)))),
    //         Rc::new(Stmt::Decl(
    //             Type::IntT,
    //             Name("true".to_string()),
    //             Some(Rc::new(Expr::Val(Rc::new(Value::IntV(1))))),
    //         )),
    //         Some(Rc::new(Stmt::Decl(
    //             Type::IntT,
    //             Name("false".to_string()),
    //             Some(Rc::new(Expr::Val(Rc::new(Value::IntV(0))))),
    //         ))),
    //     ));
    //     // Initial config
    //     let sigma_0 = Configuration {
    //         c: Control::AstStmt(Rc::clone(&ast)),
    //         e: Rc::new(HashMap::new()),
    //         s: Rc::new(HashMap::new()),
    //         k: Rc::new(Kont::Mt),
    //     };
    //     //println!("{:?}", sigma_0);
    //     let sigma_1 = sigma_0.next();
    //     //println!("{:?}", sigma_1);
    //     let sigma_2 = sigma_1.next();
    //     //println!("{:?}", sigma_2);
    // }
    // #[test]
    // fn if_false_test() {
    //     // Test ast
    //     let ast = Rc::new(Stmt::If(
    //         Rc::new(Expr::Val(Rc::new(Value::BoolV(false)))),
    //         Rc::new(Stmt::Decl(
    //             Type::IntT,
    //             Name("true".to_string()),
    //             Some(Rc::new(Expr::Val(Rc::new(Value::IntV(1))))),
    //         )),
    //         Some(Rc::new(Stmt::Decl(
    //             Type::IntT,
    //             Name("false".to_string()),
    //             Some(Rc::new(Expr::Val(Rc::new(Value::IntV(0))))),
    //         ))),
    //     ));
    //     // Initial config
    //     let sigma_0 = Configuration {
    //         c: Control::AstStmt(Rc::clone(&ast)),
    //         e: Rc::new(HashMap::new()),
    //         s: Rc::new(HashMap::new()),
    //         k: Rc::new(Kont::Mt),
    //     };
    //     println!("{:?}", sigma_0);
    //     let sigma_1 = sigma_0.next();
    //     println!("{:?}", sigma_1);
    //     let sigma_2 = sigma_1.next();
    //     println!("{:?}", sigma_2);
    // }
    // #[test]
    // fn expr_stmt_test() {
    //     // Test ast
    //     let ast = Rc::new(
    //     	Stmt::S
    //     );
    //     // Initial config
    //     let sigma_0 = Configuration {
    //         c: Control::AstStmt(Rc::clone(&ast)),
    //         e: Rc::new(HashMap::new()),
    //         s: Rc::new(HashMap::new()),
    //         k: Rc::new(Kont::Mt),
    //     };
    //     println!("{:?}", sigma_0);
    //     let sigma_1 = sigma_0.next();
    //     println!("{:?}", sigma_1);
    //     let sigma_2 = sigma_1.next();
    //     println!("{:?}", sigma_2);
    // }
}
