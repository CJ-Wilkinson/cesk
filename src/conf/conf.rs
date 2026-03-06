use crate::ast::{Expr, Name, Operation, Stmt, Value};
use Control::*;
use Expr::*;
use Kont::*;
use Stmt::*;
use Value::*;
use std::collections::HashMap;
use std::convert::From;
use std::rc::Rc;

#[derive(Debug)]
struct Address;

#[derive(Debug)]
enum Control {
    AstExpr(Rc<Expr>),
    AstStmt(Rc<Stmt>),
}

type Env = HashMap<Name, Address>;
type Store = HashMap<Address, Value>;

#[derive(Debug, PartialEq)]
enum Kont {
    Mt,
    ExprStmtK(Rc<Kont>),
    OpK(Operation, Rc<Expr>, Rc<Kont>),
    IfK(Rc<Stmt>, Option<Rc<Stmt>>, Rc<Stmt>, Rc<Kont>), // Missing successor!
}

#[derive(Debug)]
struct Config {
    c: Control,
    e: Rc<Env>,
    s: Rc<Store>,
    k: Rc<Kont>,
}

impl From<Stmt> for Config {
    fn from(s: Stmt) -> Self {
        Self {
            c: AstStmt(Rc::new(s)),
            e: Rc::new(HashMap::new()),
            s: Rc::new(HashMap::new()),
            k: Rc::new(Mt),
        }
    }
}

fn successor_lookup(_key: Rc<Stmt>) -> Rc<Stmt> {
    Rc::new(Break)
}

impl From<Expr> for Config {
    fn from(e: Expr) -> Self {
        Self {
            c: AstExpr(Rc::new(e)),
            e: Rc::new(HashMap::new()),
            s: Rc::new(HashMap::new()),
            k: Rc::new(Mt),
        }
    }
}

impl Config {
    fn no_change_map(&self, c: Control, k: Kont) -> Config {
        Self {
            c,
            e: Rc::clone(&self.e),
            s: Rc::clone(&self.s),
            k: Rc::new(k),
        }
    }
    fn new_c(&self, c: Control) -> Config {
        Self {
            c,
            e: Rc::clone(&self.e),
            s: Rc::clone(&self.s),
            k: Rc::clone(&self.k),
        }
    }
    fn next(&self) -> Self {
        // Match control
        match &self.c {
            AstStmt(s) => {
                // Match on statement
                match s.as_ref() {
                    // Expression Statement
                    ExprStmt(expr) => Self {
                        c: AstExpr(Rc::clone(expr)),
                        e: Rc::clone(&self.e),
                        s: Rc::clone(&self.s),
                        k: Rc::new(ExprStmtK(Rc::clone(&self.k))),
                    },
                    // If statement
                    If(expr, true_b, false_b) => Self {
                        c: AstExpr(Rc::clone(expr)),
                        e: Rc::clone(&self.e),
                        s: Rc::clone(&self.s),
                        k: Rc::new(IfK(
                            Rc::clone(true_b),
                            match false_b {
                                Some(false_b) => Some(Rc::clone(false_b)),
                                None => None,
                            },
                            Rc::new(Break), // TODO: Successor function
                            Rc::clone(&self.k),
                        )),
                    },
                    _ => todo!(),
                }
            }
            AstExpr(e) => match e.as_ref() {
                Op(l, op, r) => {
                    if let (Val(l), Val(r)) = (l.as_ref(), r.as_ref()) {
                        self.new_c(AstExpr(Rc::new(Val(Rc::new(op.call(l, r))))))
                    } else {
                        self.no_change_map(
                            AstExpr(Rc::clone(l)),
                            OpK(*op, Rc::clone(r), Rc::clone(&self.k)),
                        )
                    }
                }
                Val(v) => self.invoke_kont(v),
                _ => todo!(),
            },
        }
    }
    fn invoke_kont(&self, v1: &Rc<Value>) -> Config {
        match self.k.as_ref() {
            OpK(op, expr, k) => {
                // Is the expression a value?
                match expr.as_ref() {
                    Val(v2) => Self {
                        c: AstExpr(Rc::new(Val(Rc::new(op.call(v1, v2))))),
                        e: Rc::clone(&self.e),
                        s: Rc::clone(&self.s),
                        k: Rc::clone(k),
                    },
                    _ => Self {
                        c: AstExpr(Rc::clone(expr)),
                        e: Rc::clone(&self.e),
                        s: Rc::clone(&self.s),
                        k: Rc::new(OpK(
                            *op,
                            match &self.c {
                                AstExpr(expr) => Rc::clone(expr),
                                _ => panic!(), // Unreachable?
                            },
                            Rc::clone(k),
                        )),
                    },
                }
            }
            IfK(true_b, false_b, succ, k) => Self {
                c: AstStmt(Rc::clone(match v1.as_ref() {
                    BoolV(true) => true_b,
                    BoolV(false) => match false_b {
                        Some(false_b) => false_b,
                        None => succ,
                    },
                    _ => panic!(),
                })),
                e: Rc::clone(&self.e),
                s: Rc::clone(&self.s),
                k: Rc::clone(k),
            },
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn is_terminal(conf: &Config) -> Option<Rc<Value>> {
        match &conf.c {
            AstExpr(e) => match e.as_ref() {
                Val(v) => {
                    if *conf.k == Mt {
                        Some(Rc::clone(v))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }

    #[test]
    fn arith_test() {
        // Not a real test. Runs a basic arithmetic expression
        let ast = Op(
            Rc::new(Val(Rc::new(IntV(9)))),
            Operation::Add,
            Rc::new(Op(
                Rc::new(Val(Rc::new(IntV(27)))),
                Operation::Div,
                Rc::new(Val(Rc::new(IntV(9)))),
            )),
        );
        let mut conf = Config::from(ast);
        loop {
            println!("{:?}", conf);
            conf = conf.next();
            match is_terminal(&conf) {
                Some(v) => {
                    println!("Got: {:?}", v);
                    assert_eq!(IntV(12), *v);
                    return;
                }
                None => (),
            }
        }
    }
}
