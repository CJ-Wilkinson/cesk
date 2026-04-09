use crate::ast::*;

use super::parts::control::Control;
use Control::*;
use chumsky::prelude::todo;
use super::parts::environment::Env;
use super::parts::store::Store;
use super::parts::kont::Kont;
use Kont::*;
use super::parts::address::Address;
use Expr::*;

use Stmt::*;
use Value::*;

use core::prelude::v1;
use std::collections::HashMap;
use std::convert::From;
use std::fmt;
use std::ops::Neg;
use std::rc::Rc;

use super::prog_handler::ProgramHandler;

#[derive(Debug)]
pub struct Config {
    c: Control,
    e: Rc<Env>,
    s: Rc<Store>,
    k: Rc<Kont>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "<{}, {}, {}, {:?}>", self.c, self.e, self.s, self.k)
    }
}

impl From<&Rc<Stmt>> for Config { // * Config helper
    fn from(s: &Rc<Stmt>) -> Self {
        Self {
            c: AstStmt(s.clone()),
            e: Rc::new(Env::new()),
            s: Rc::new(Store::new()),
            k: Rc::new(Mt),
        }
    }
}

impl From<Expr> for Config { // * Config helper
    fn from(e: Expr) -> Self {
        Self {
            c: AstExpr(Rc::new(e)),
            e: Rc::new(Env::new()),
            s: Rc::new(Store::new()),
            k: Rc::new(Mt),
        }
    }
}

impl Config {
    pub fn next(&self, handler: &mut ProgramHandler) -> Self {
        // Match control
        match &self.c {
            AstStmt(s) => {
                // Match on statement
                match s.as_ref() {
                    // Expression Statement
                    ExprStmt(expr) => Self {
                        c: AstExpr(expr.clone()),
                        e: self.e.clone(),
                        s: self.s.clone(),
                        k: Rc::new(ExprStmtK(
                            handler.successor_lookup(s.clone()),
                            self.k.clone()
                        )),
                    },
                    // If statement
                    If(expr, true_b, false_b) => Self {
                        c: AstExpr(expr.clone()),
                        e: self.e.clone(),
                        s: self.s.clone(),
                        k: Rc::new(IfK(
                            true_b.clone(),
                            match false_b {
                                Some(false_b) => Some(false_b.clone()),
                                None => None,
                            },
                            handler.successor_lookup(s.clone()),
                            self.k.clone(),
                        )),
                    },
                    DeclD(_, lval, expr) => Self { // ! Get rid of
                        c: match expr {
                            Some(expr) => AstExpr(expr.clone()),
                            None => AstExpr(Rc::new(Val(Rc::new(UnitV)))),
                        },
                        e: self.e.clone(),
                        s: self.s.clone(),
                        k: Rc::new(DeclK(
                            lval.clone(), // TODO should be copy?
                            handler.successor_lookup(s.clone()),
                            self.k.clone(),
                        )),
                    },
                    Decl(lval) => Self {
                        /* Introduce variable into environment */
                        c: AstExpr(Rc::new(Val(Rc::new(UnitV)))),
                        e: {
                            let mut new_env = (*self.e).clone();
                            new_env.0.insert(lval.clone(), handler.get_address());
                            Rc::new(new_env)
                        },
                        s: self.s.clone(),
                        k: Rc::new(DeclK(
                            lval.clone(),
                            handler.successor_lookup(s.clone()),
                            self.k.clone(),
                        )),
                    },
                    Assign(lval, rval) => Self {
                        c: AstExpr(rval.clone()),
                        e: self.e.clone(),
                        s: self.s.clone(),
                        k: Rc::new(AssignK(
                            lval.clone(),
                            handler.successor_lookup(s.clone()),
                            self.k.clone(),
                        )),
                    },
                    Return(expr) => match self.k.as_ref() {
                        BlocK(_, _, k) => Self {
                            c: AstStmt(s.clone()),
                            e: self.e.clone(),
                            s: self.s.clone(),
                            k: k.clone(),
                        },
                        ReturnK(_, _) => Self {
                            c: AstExpr(expr.clone()),
                            e: self.e.clone(),
                            s: self.s.clone(),
                            k: self.k.clone(),
                        },
                        FunK(env,k ) => Self {
                            c: AstExpr(expr.clone()),
                            e: self.e.clone(),
                            s: self.s.clone(),
                            k: Rc::new(ReturnK(
                                env.clone(),
                                k.clone()
                            )),
                        },
                        Mt => Self {
                            c: AstExpr(expr.clone()),
                            e: self.e.clone(),
                            s: self.s.clone(),
                            k: self.k.clone(),
                        },
                        _ => panic!("Found some other Kont"),
                    },
                    Block(stmts) => match stmts.get(0) {
                        Some(stmt) => Self {
                            c: AstStmt(stmts.get(0).unwrap().clone()), // TODO unwrap
                            e: self.e.clone(),
                            s: self.s.clone(),
                            k: Rc::new(BlocK(
                                self.e.clone(),
                                handler.successor_lookup(s.clone()),
                                self.k.clone(),
                            ))
                        },
                        None => Self {
                            c: AstStmt(handler.successor_lookup(s.clone())),
                            e: self.e.clone(),
                            s: self.s.clone(),
                            k: self.k.clone(),
                        }
                    },
                    Break => match self.k.as_ref(){
                        WhileK(env,_cond ,_body ,succ ,k ) => Self {
                            c: AstStmt(succ.clone()),
                            e: env.clone(),
                            s: self.s.clone(),
                            k: k.clone(),
                        },
                        BlocK(env, succ, k ) => Self {
                            c: AstStmt(s.clone()),
                            e: env.clone(),
                            s: self.s.clone(),
                            k: k.clone()
                        },
                        k => panic!("Found some other Kont : {k:?}"),
                    }, 
                    While(cond,body) => Self {
                        c: AstExpr(cond.clone()),
                        e: self.e.clone(),
                        s: self.s.clone(),
                        k: Rc::new(WhileK(
                            self.e.clone(),
                            cond.clone(),
                            body.clone(),
                            handler.successor_lookup(s.clone()),
                            self.k.clone(),
                        )),
                    },
                    Continue => match self.k.as_ref() {
                        WhileK(env,cond,_,_,k) => Self {
                            c: AstExpr(cond.clone()),
                            e: env.clone(),
                            s: self.s.clone(),
                            k: self.k.clone(),
                        },
                        BlocK(env,_,k) => Self {
                            c: AstStmt(s.clone()),
                            e: env.clone(),
                            s: self.s.clone(),
                            k: k.clone(),
                        },
                        k => panic!("Found some other Kont : {k:?}"),
                    },
                    WhileD(_, _) => panic!("WhileD found in "), // ! Will be removed later
                    ForD(_, _, _, _) => todo!() // ! Will be removed later
                }
            }
            AstExpr(e) => match e.as_ref() {
                BinaryOp(l, op, r) => {
                    if let (Val(l), Val(r)) = (l.as_ref(), r.as_ref()) {
                        Self {
                            c: AstExpr(Rc::new(Val(Rc::new(op.call(l, r))))),
                            e: self.e.clone(),
                            s: self.s.clone(),
                            k: self.k.clone()
                        }
                    } else {
                        Self {
                            c: AstExpr(l.clone()),
                            e: self.e.clone(),
                            s: self.s.clone(),
                            k: Rc::new(OpK(
                                *op,
                                r.clone(),
                                self.k.clone()
                            )),
                        }
                    }
                }
                Array(exprs) => {
                    // Get the first address
                    let addr = handler.get_address();
                    // Build the array handler value
                    let array_ref = Value::ArrayV(exprs.len(), addr.clone());
                    // Get new store
                    let mut new_store = (*self.s).clone();

                    // Bind the first item in array to addrss
                    match exprs.first() {
                        Some(expr) => {
                            if let Val(v) = expr.as_ref() {
                                new_store.insert(addr, v.clone())
                            }
                        }
                        None => (),
                    }
                    for expr in exprs.iter().skip(1) {
                        if let Val(v) = expr.as_ref() {
                            new_store.insert(handler.get_address(), v.clone());
                        }
                    }
                    // Make new environment
                    // Place the handler in the control
                    Self {
                        c: AstExpr(Rc::new(Val(Rc::new(array_ref)))),
                        e: self.e.clone(),
                        s: Rc::new(new_store),
                        k: self.k.clone(),
                    }
                }
                Var(name) => match self.e.get(name) {
                    Some(addr) => Self {
                        c: AstExpr(Rc::new(Val(Rc::new(AddrV(addr.clone()))))),
                        e: self.e.clone(),
                        s: self.s.clone(),
                        k: self.k.clone(),
                    },
                    None => panic!("Undefined variable: {name}"),
                },
                CallRef(fun, args) => match args.slice_ref() { 
                    [first, rest @ ..] => Self {
                        c: AstExpr(first.clone()),
                        e: Rc::new(Env::new()),
                        s: self.s.clone(),
                        k: Rc::new(CallK(
                            self.e.clone(),
                            fun.clone(),
                            fun.params.clone(),
                            Rc::new(Arguments::from(rest)),
                            self.k.clone(),
                        ))
                    },
                    [] => Self {
                            c: AstStmt(fun.body.clone()),
                            e: Rc::new(Env::new()),
                            s: self.s.clone(),
                            k: Rc::new(FunK(
                                self.e.clone(),
                                self.k.clone(),
                            )),
                    },
                },
                Val(v) => self.invoke_kont(v.clone(), handler),
                CallName(name,_) => panic!("CallName expression encountered {name}"),
                Neg(_) => todo!(), // ! Will get desugared to 0 - val
                Index(_, _) => todo!(),
            },
            Addr(_) => todo!(), // ? When?
        }
    }
    fn invoke_kont(&self, v1: Rc<Value>, handler: &mut ProgramHandler) -> Config {
        match self.k.as_ref() {
            OpK(op, expr, k) => {
                // Is the expression a value?
                match expr.as_ref() {
                    Val(v2) => Self {
                        c: AstExpr(Rc::new(Val(Rc::new(op.call(v2, &v1))))),
                        e: self.e.clone(),
                        s: self.s.clone(),
                        k: k.clone(),
                    },
                    _ => Self {
                        c: AstExpr(expr.clone()),
                        e: self.e.clone(),
                        s: self.s.clone(),
                        k: Rc::new(OpK(
                            *op,
                            Rc::new(Expr::Val(v1.clone())),
                            k.clone(),
                        )),
                    },
                }
            },
            IfK(true_b, false_b, succ, k) => Self {
                c: AstStmt((match v1.as_ref() {
                    BoolV(true) => true_b,
                    BoolV(false) => match false_b {
                        Some(false_b) => false_b,
                        None => succ,
                    },
                    _ => panic!(),
                }).clone()),
                e: self.e.clone(),
                s: self.s.clone(),
                k: k.clone(),
            },
            DeclK(lval, succ, k) => {
                // Get new address
                let addr = handler.get_address();

                Self {
                    c: AstStmt(succ.clone()),
                    e: {
                        let mut new_env = (*self.e).clone();
                        new_env.0.insert(lval.clone(), addr.clone());
                        Rc::new(new_env)
                    },
                    s: {
                        let mut new_store = (*self.s).clone();
                        new_store.insert(addr.clone(), v1.clone());
                        Rc::new(new_store)
                    },
                    k: k.clone(),
                }
            }
            ReturnK(env, k) => Self {
                c: AstExpr(Rc::new(Expr::Val(v1.clone()))),
                e: env.clone(),
                s: self.s.clone(),
                k: k.clone(),
            },
            AssignK(lval, succ, k) => { // TODO Redo this 
                let addr: &Address = match lval.as_ref() {
                    Var(n) => {
                        if let Some(addr) = self.e.0.get(n) {
                            addr
                        } else {
                            panic!()
                        }
                    }
                    _ => todo!(), // TODO Array indexing
                };
                Self {
                    c: AstStmt(succ.clone()),
                    e: self.e.clone(),
                    s: {
                        let mut new_store = (*self.s).clone();
                        new_store.insert(addr.clone(), v1.clone());
                        Rc::new(new_store)
                    },
                    k: k.clone(),
                }
            }
            BlocK(env, succ, k) => Self {
                c: AstStmt(succ.clone()),
                e: env.clone(),
                s: self.s.clone(),
                k: k.clone(),
            },
            CallK(env, fun, params, args, k) => {
                match (args.slice_ref(), params.slice_ref()) {
                    ([first, rest @ ..], [pfirst, prest @ ..]) => Self {
                        c: AstExpr(first.clone()),
                        e: Rc::new(Env::new()),
                        s: self.s.clone(),
                        k: Rc::new(CallK(
                            self.e.clone(),
                            fun.clone(),
                            fun.params.clone(),
                            Rc::new(Arguments::from(rest)),
                            self.k.clone(),
                        ))
                    },
                    ([], []) => Self {
                            c: AstStmt(fun.body.clone()),
                            e: Rc::new(Env::new()),
                            s: self.s.clone(),
                            k: Rc::new(FunK(
                                self.e.clone(),
                                self.k.clone(),
                            )),
                    },
                    _ => panic!("Mismatched number of arguments and paramenters"),
                }
            },
            ExprStmtK(succ, k) => Self {
                c: AstStmt(succ.clone()),
                e: self.e.clone(),
                s: self.s.clone(),
                k: k.clone()
            },
            WhileK(env, cond, body, succ, k) => match v1.as_ref() {
                BoolV(true) => Self {
                    c: AstStmt(body.clone()),
                    e: self.e.clone(),
                    s: self.s.clone(),
                    k: self.k.clone(),
                },
                BoolV(false) => Self {
                    c: AstStmt(succ.clone()),
                    e: env.clone(),
                    s: self.s.clone(),
                    k: k.clone(),
                },
                _ => panic!("Non-Boolean found in condition")
            },
            FunK(_, _) => todo!(),
            IdK(_,_ ,_ ) => todo!(),
            Mt => panic!("Exited with code {v1}"),
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
                        Some(v.clone())
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }

    // #[test]
    // fn arith_test() {
    //     // Not a real test. Runs a basic arithmetic expression
    //     // let ast = Op(
    //     //     Rc::new(Val(Rc::new(IntV(9)))),
    //     //     Operation::Add,
    //     //     Rc::new(Op(
    //     //         Rc::new(Val(Rc::new(IntV(27)))),
    //     //         Operation::Div,
    //     //         Rc::new(Val(Rc::new(IntV(9)))),
    //     //     )),
    //     // );
    //     let ast = If(
    //         Rc::new(BinaryOp(
    //             Rc::new(Val(Rc::new(IntV(3)))),
    //             Operation::Lt,
    //             Rc::new(Val(Rc::new(IntV(4)))),
    //         )),
    //         Rc::new(DeclD(
    //             Type::IntT,
    //             Name("Hi".to_string()),
    //             Some(Rc::new(BinaryOp(
    //                 Rc::new(Val(Rc::new(IntV(9)))),
    //                 Operation::Add,
    //                 Rc::new(BinaryOp(
    //                     Rc::new(Val(Rc::new(IntV(27)))),
    //                     Operation::Div,
    //                     Rc::new(Val(Rc::new(IntV(9)))),
    //                 )),
    //             ))),
    //         )),
    //         None,
    //     );
    //     let mut conf = Config::from(ast);
    //     loop {
    //         println!("{}", conf);
    //         //print!("c: {}, e: {}, s: {}, k: {}")
    //         conf = conf.next();
    //         match is_terminal(&conf) {
    //             Some(v) => {
    //                 println!("Got: {:?}", v);
    //                 assert_eq!(IntV(12), *v);
    //                 return;
    //             }
    //             None => (),
    //         }
    //     }
    // }
}
