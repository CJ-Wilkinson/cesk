use crate::ast::{ArithBinop, Expr, Name, Stmt, Value};
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
    ArithK(ArithBinop, Rc<Expr>, Rc<Kont>),
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
                    ExprStmt(expr) => Self {
                        c: AstExpr(Rc::clone(expr)),
                        e: Rc::clone(&self.e),
                        s: Rc::clone(&self.s),
                        k: Rc::new(ExprStmtK(Rc::clone(&self.k))),
                    },
                    _ => todo!(),
                }
            }
            AstExpr(e) => match e.as_ref() {
                ArithOp(l, op, r) => {
                    if let (Val(l), Val(r)) = (l.as_ref(), r.as_ref()) {
                        self.new_c(AstExpr(Rc::new(Val(Rc::new(op.call(l, r))))))
                    } else {
                        self.no_change_map(
                            AstExpr(Rc::clone(l)),
                            ArithK(*op, Rc::clone(r), Rc::clone(&self.k)),
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
            ArithK(op, expr, k) => {
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
                        k: Rc::new(ArithK(
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
        let ast = ArithOp(
            Rc::new(Val(Rc::new(IntV(9)))),
            ArithBinop::Add,
            Rc::new(ArithOp(
                Rc::new(Val(Rc::new(IntV(10)))),
                ArithBinop::Mult,
                Rc::new(Val(Rc::new(IntV(11)))),
            )),
        );
        let mut conf = Config::from(ast);
        loop {
            //println!("{:?}", conf);
            conf = conf.next();
            match is_terminal(&conf) {
                Some(v) => {
                	println!("Got: {:?}", v);
                	assert_eq!(IntV(119), *v);
                	return;
                },
                None => (),
            }
        }
    }
}
