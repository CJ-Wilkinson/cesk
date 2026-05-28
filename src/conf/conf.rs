use crate::ast::*;

use super::parts::address::Address;
use super::parts::control::Control;
use super::parts::environment::Env;
use super::parts::kont::Kont;
use super::parts::store::Store;
use Control::*;
use Expr::*;
use Kont::*;
use chumsky::prelude::todo;

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
        writeln!(f, "<{}, {:?}, {:?}, {:?}>", self.c, self.e, self.s, self.k)
    }
}

impl From<&Rc<Stmt>> for Config {
    // * Config helper
    fn from(s: &Rc<Stmt>) -> Self {
        Self {
            c: AstStmt(s.clone()),
            e: Rc::new(Env::new()),
            s: Rc::new(Store::new()),
            k: Rc::new(Mt),
        }
    }
}

impl From<Expr> for Config {
    // * Config helper
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
                    ExprStmt { expr } => Self {
                        c: AstExpr(expr.clone()),
                        e: self.e.clone(),
                        s: self.s.clone(),
                        k: Rc::new(ExprStmtK(
                            handler.successor_lookup(s.clone()),
                            self.k.clone(),
                        )),
                    },
                    // If statement
                    If {
                        condition: expr,
                        then_branch: true_b,
                        else_branch: false_b,
                    } => Self {
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
                    Decl { name: id, .. } => {
                        // todo *** Need to figure out how we're handling types. Parser takes types as input
                        let addr = handler.get_address();
                        Self {
                            /* Introduce variable into environment */
                            c: AstStmt(handler.successor_lookup(s.clone())),
                            e: {
                                let mut new_env = (*self.e).clone();
                                new_env.insert(id.clone(), addr.clone());
                                Rc::new(new_env)
                            },
                            s: {
                                let mut new_store = (*self.s).clone();
                                new_store.insert(addr, Rc::new(Value::UnitV));
                                Rc::new(new_store)
                            },
                            k: self.k.clone(),
                        }
                    }
                    Assign {
                        lhs: lval,
                        rhs: expr,
                    } => Self {
                        c: AstExpr(expr.clone()),
                        e: self.e.clone(),
                        s: self.s.clone(),
                        k: Rc::new(AssignK(
                            lval.clone(),
                            handler.successor_lookup(s.clone()),
                            self.k.clone(),
                        )),
                    },
                    Return { expr } => match self.k.as_ref() {
                        BlockK(_, _, k) => Self {
                            c: AstStmt(s.clone()),
                            e: self.e.clone(),
                            s: self.s.clone(),
                            k: k.clone(),
                        },
                        ReturnK(_, _, _) => Self {
                            c: AstExpr(expr.clone()),
                            e: self.e.clone(),
                            s: self.s.clone(),
                            k: self.k.clone(),
                        },
                        FunK(typ, env, k) => Self {
                            c: AstExpr(expr.clone()),
                            e: self.e.clone(),
                            s: self.s.clone(),
                            k: Rc::new(ReturnK(typ.clone(), env.clone(), k.clone())),
                        },
                        Mt => Self {
                            c: AstExpr(expr.clone()),
                            e: self.e.clone(),
                            s: self.s.clone(),
                            k: self.k.clone(),
                        },
                        _ => panic!("Found some other Kont"),
                    },
                    Block { stmts } => match stmts.get(0) {
                        // TODO: fix this
                        Some(stmt) => Self {
                            c: AstStmt(stmts.get(0).unwrap().clone()), // TODO unwrap
                            e: self.e.clone(),
                            s: self.s.clone(),
                            k: Rc::new(BlockK(
                                self.e.clone(),
                                handler.successor_lookup(s.clone()),
                                self.k.clone(),
                            )),
                        },
                        None => Self {
                            c: AstStmt(handler.successor_lookup(s.clone())),
                            e: self.e.clone(),
                            s: self.s.clone(),
                            k: self.k.clone(),
                        },
                    },
                    Break => match self.k.as_ref() {
                        WhileK(env, _cond, _body, succ, k) => Self {
                            c: AstStmt(succ.clone()),
                            e: env.clone(),
                            s: self.s.clone(),
                            k: k.clone(),
                        },
                        BlockK(env, succ, k) => Self {
                            c: AstStmt(s.clone()),
                            e: env.clone(),
                            s: self.s.clone(),
                            k: k.clone(),
                        },
                        k => panic!("Found some other Kont : {k:?}"),
                    },
                    While {
                        condition: cond,
                        body,
                    } => Self {
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
                        WhileK(env, cond, _, _, k) => Self {
                            c: AstExpr(cond.clone()),
                            e: env.clone(),
                            s: self.s.clone(),
                            k: self.k.clone(),
                        },
                        BlockK(env, _, k) => Self {
                            c: AstStmt(s.clone()),
                            e: env.clone(),
                            s: self.s.clone(),
                            k: k.clone(),
                        },
                        k => panic!("Found some other Kont : {k:?}"),
                    },
                    ForD {
                        init: _,
                        condition: _,
                        update: _,
                        body: _,
                    } => unreachable!("Found ForD in Control"),
                }
            }
            AstExpr(e) => match e.as_ref() {
                BinaryOp { lhs: l, op, rhs: r } => Self {
                    c: AstExpr(l.clone()),
                    e: self.e.clone(),
                    s: self.s.clone(),
                    k: Rc::new(OpLK(*op, r.clone(), self.k.clone())),
                },
                UnaryOp { op, expr: exp } => Self {
                    c: AstExpr(exp.clone()),
                    e: self.e.clone(),
                    s: self.s.clone(),
                    k: Rc::new(UOpK(*op, self.k.clone())),
                },
                Array { elements: exprs } => {
                    // Get the first address
                    let addr = handler.get_address();
                    // Build the array handler value
                    let array_ref = Value::ArrayV(exprs.len(), addr.clone());
                    // Get new store
                    let mut new_store = (*self.s).clone();

                    // Bind the first item in array to addrss
                    match exprs.first() {
                        Some(expr) => {
                            if let Val { value: v } = expr.as_ref() {
                                new_store.insert(addr, v.clone());
                            }
                        }
                        None => (),
                    }
                    for expr in exprs.iter().skip(1) {
                        if let Val { value: v } = expr.as_ref() {
                            new_store.insert(handler.get_address(), v.clone());
                        }
                    }
                    // Make new environment
                    // Place the handler in the control
                    Self {
                        c: AstExpr(Rc::new(Expr::val(Rc::new(array_ref)))),
                        e: self.e.clone(),
                        s: Rc::new(new_store),
                        k: self.k.clone(),
                    }
                }
                Var { name: id } => match self.e.get(id) {
                    Some(addr) => Self {
                        c: AstExpr(Rc::new(Expr::val(Rc::new(AddrV(addr.clone()))))),
                        e: self.e.clone(),
                        s: self.s.clone(),
                        k: self.k.clone(),
                    },
                    None => panic!("Undefined variable: {id}"),
                },
                Index {
                    array: id,
                    index: expr,
                } => Self {
                    c: AstExpr(expr.clone()),
                    e: self.e.clone(),
                    s: self.s.clone(),
                    k: Rc::new(IndexK(Rc::new(id.clone()), self.k.clone())),
                },
                // CallRef { fun, args } => match args.slice_ref() {
                //     [first, rest @ ..] => Self {
                //         c: AstExpr(first.clone()),
                //         e: Rc::new(Env::new()),
                //         s: self.s.clone(),
                //         k: Rc::new(CallK(
                //             self.e.clone(),
                //             fun.clone(),
                //             fun.params.clone(),
                //             Rc::new(Arguments::from(rest)),
                //             self.k.clone(),
                //         )),
                //     },
                //     [] => Self {
                //         c: AstStmt(fun.body.clone()),
                //         e: Rc::new(Env::new()),
                //         s: self.s.clone(),
                //         k: Rc::new(FunK(self.e.clone(), self.k.clone())),
                //     },
                // },
                CallRef { fun: _, args: _ } => todo!(),

                Val { value: v } => {
                    if let Value::AddrV(a) = v.as_ref() {
                        if let LvalK(_, _, _) = self.k.as_ref() {
                            self.invoke_kont(v.clone(), handler)
                        } else {
                            Self {
                                c: AstExpr(Rc::new(Expr::val(
                                    self.s.get(a).expect("Address not found in store.").clone(),
                                ))),
                                e: self.e.clone(),
                                s: self.s.clone(),
                                k: self.k.clone(),
                            }
                        }
                    } else {
                        self.invoke_kont(v.clone(), handler)
                    }
                }
                //
                //                 CallName {
                //                     callee: name,
                //                     args: _,
                //                 } => unreachable!("CallName expression encountered: '{name}'"),
                CallName { callee, args } => {
                    if let Fun {
                        typ,
                        name,
                        params,
                        body,
                    } = &*handler.function_lookup(&callee).unwrap()
                    {
                        match args.slice_ref() {
                            [first, rest @ ..] => Self {
                                c: AstExpr(first.clone()),
                                e: self.e.clone(),
                                s: self.s.clone(),
                                k: Rc::new(CallK(
                                    typ.clone(),
                                    self.e.clone(),
                                    Rc::new(Env::new()),
                                    body.clone(),
                                    params.clone(),
                                    Rc::new(Arguments::from(rest)),
                                    self.k.clone(),
                                )),
                            },
                            [] => Self {
                                c: AstStmt(body.clone()),
                                e: Rc::new(Env::new()),
                                s: self.s.clone(),
                                k: Rc::new(FunK(typ.clone(), self.e.clone(), self.k.clone())),
                            },
                        }
                    } else {
                        panic!()
                    }
                }
            },
        }
    }
    fn invoke_kont(&self, v1: Rc<Value>, handler: &mut ProgramHandler) -> Config {
        match self.k.as_ref() {
            // needs to be above AddressLookup yea sure perfect ok I just love this
            LvalK(val, succ, k) => Self {
                c: AstStmt(succ.clone()),
                e: self.e.clone(),
                s: {
                    if let Value::AddrV(addr) = v1.as_ref() {
                        let mut new_store = (*self.s).clone();
                        new_store.insert(addr.clone(), val.clone());
                        Rc::new(new_store)
                    } else {
                        panic!("Encountered a non-address")
                    }
                },
                k: k.clone(),
            },
            OpLK(op, expr, k) => Self {
                c: AstExpr(expr.clone()),
                e: self.e.clone(),
                s: self.s.clone(),
                k: Rc::new(OpRK(*op, v1.clone(), k.clone())),
            },
            OpRK(op, v2, k) => Self {
                c: AstExpr(Rc::new(Expr::val(Rc::new(op.call(v2, &v1))))),
                e: self.e.clone(),
                s: self.s.clone(),
                k: k.clone(),
            },
            UOpK(op, k) => Self {
                c: AstExpr(Rc::new(Expr::val(Rc::new(op.call(&v1))))),
                e: self.e.clone(),
                s: self.s.clone(),
                k: k.clone(),
            },
            IfK(true_b, false_b, succ, k) => Self {
                c: AstStmt(
                    (match v1.as_ref() {
                        BoolV(true) => true_b,
                        BoolV(false) => match false_b {
                            Some(false_b) => false_b,
                            None => succ,
                        },
                        _ => panic!(),
                    })
                    .clone(),
                ),
                e: self.e.clone(),
                s: self.s.clone(),
                k: k.clone(),
            },
            ReturnK(typ, env, k) => {
                if &v1.get_type() == typ {
                    Self {
                        c: AstExpr(Rc::new(Expr::val(v1.clone()))),
                        e: env.clone(),
                        s: self.s.clone(),
                        k: k.clone(),
                    }
                } else {
                    panic!("return type mismatch")
                }
            }
            AssignK(lval, succ, k) => Self {
                c: AstExpr(lval.clone()),
                e: self.e.clone(),
                s: self.s.clone(),
                k: Rc::new(LvalK(v1.clone(), succ.clone(), k.clone())),
            },
            BlockK(env, succ, k) => Self {
                c: AstStmt(succ.clone()),
                e: env.clone(),
                s: self.s.clone(),
                k: k.clone(),
            },
            CallK(typ, old_env, fun_env, body, params, args, k) => {
                let addr = handler.get_address();

                match (args.slice_ref(), params.slice_ref()) {
                    ([first, rest @ ..], [pfirst, prest @ ..]) => {
                        if v1.get_type() == pfirst.typ {
                            Self {
                                c: AstExpr(first.clone()),
                                e: self.e.clone(),
                                s: {
                                    let mut new_store = (*self.s).clone();
                                    new_store.insert(addr.clone(), v1.clone());
                                    Rc::new(new_store)
                                },
                                k: Rc::new(CallK(
                                    typ.clone(),
                                    old_env.clone(),
                                    {
                                        let mut new_env = fun_env.as_ref().clone();
                                        new_env.insert(pfirst.name.clone(), addr.clone());
                                        Rc::new(new_env)
                                    },
                                    body.clone(),
                                    Rc::new(ParamList::from(prest)),
                                    Rc::new(Arguments::from(rest)),
                                    k.clone(),
                                )),
                            }
                        } else {
                            panic!(
                                "type mismatch: found {}, expected {}",
                                v1.get_type(),
                                pfirst.typ
                            );
                        }
                    }
                    ([], [pfirst]) => {
                        if v1.get_type() == pfirst.typ {
                            Self {
                                c: AstStmt(body.clone()),
                                e: {
                                    let mut new_env = fun_env.as_ref().clone();
                                    new_env.insert(pfirst.name.clone(), addr.clone());
                                    Rc::new(new_env)
                                },
                                s: {
                                    let mut new_store = (*self.s).clone();
                                    new_store.insert(addr.clone(), v1.clone());
                                    Rc::new(new_store)
                                },
                                k: Rc::new(FunK(typ.clone(), old_env.clone(), k.clone())),
                            }
                        } else {
                            panic!(
                                "type mismatch: found {}, expected {}",
                                v1.get_type(),
                                pfirst.typ
                            );
                        }
                    }
                    _ => panic!("mismatched number of arguments and paramenters"),
                }
            }
            ExprStmtK(succ, k) => Self {
                c: AstStmt(succ.clone()),
                e: self.e.clone(),
                s: self.s.clone(),
                k: k.clone(),
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
                _ => panic!("Non-Boolean found in condition"),
            },
            IndexK(id, k) => match self.e.get(id.as_ref()) {
                Some(addr) => Self {
                    c: AstExpr(Rc::new(Expr::val(Rc::new(AddrV(addr.clone()))))),
                    e: self.e.clone(),
                    s: self.s.clone(),
                    k: k.clone(),
                },
                None => panic!("Undefined variable: {id}"),
            },
            FunK(_, _, _) => unreachable!("Expected return, found value"),
            Mt => {
                if v1.get_type() == Type::IntT {
                    panic!("exited with code {v1}")
                } else {
                    panic!("invalid type for exit code")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn is_terminal(conf: &Config) -> Option<Rc<Value>> {
        match &conf.c {
            AstExpr(e) => match e.as_ref() {
                Val { value: v } => {
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
