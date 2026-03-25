use crate::ast::*;

use Control::*;
use Expr::*;
use Kont::*;
use Stmt::*;
use Value::*;

use std::collections::HashMap;
use std::convert::From;
use std::fmt;
use std::rc::Rc;

use super::prog_handler::ProgramHandler;

#[derive(Debug, Eq, PartialOrd, Ord, Hash, PartialEq, Clone)]
pub struct Address {
    a: usize,
}

impl Address {
    pub fn get_address(&mut self) -> Address {
        let addr = self.a;
        self.a += 1;
        Address { a: addr }
    }
    pub fn new() -> Self {
        Self { a: 0 }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.a)
    }
}

#[derive(Debug)]
enum Control {
    AstExpr(Rc<Expr>),
    AstStmt(Rc<Stmt>),
}

impl fmt::Display for Control {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AstExpr(expr) => write!(f, "{}", expr),
            AstStmt(stmt) => write!(f, "{}", stmt),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Env(pub HashMap<Name, Address>);

impl Env {
    fn new() -> Self {
        Self(HashMap::new())
    }
    fn insert(&mut self, name: Name, addr: Address) {
        self.0.insert(name, addr);
    }
    fn get(&mut self, name: &Name) -> Option<&Address> {
        self.0.get(name)
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "[")?;
        // for (index, (key, value)) in self.0.iter().enumerate() {
        //     if index == self.0.len() {
        //         write!(f, "{:?} -> {:?}", key, value)?;
        //     } else {
        //         write!(f, "{:?} -> {:?}, ", key, value)?;
        //     }
        // }
        // write!(f, "]")
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|(key, value)| format!("{} -> {}", key, value))
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Store(HashMap<Address, Rc<Value>>);

impl Store {
    fn new() -> Self {
        Self(HashMap::new())
    }
    fn insert(&mut self, addr: Address, val: Rc<Value>) {
        self.0.insert(addr, val);
    }
    fn get(&mut self, addr: &Address) -> Option<Rc<Value>> {
        match self.0.get(addr) {
            Some(val) => Some(val.clone()),
            None => None,
        }
    }
}

impl fmt::Display for Store {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|(key, value)| format!("{} -> {}", key, value))
                .collect::<Vec<_>>()
                .join(", "),
        )
        //     write!(f, "[")?;
        //     fo
        //     //let mut index = 0;
        //     for (index, (key, value)) in self.0.iter().enumerate() {
        //         if index == self.0.len() {
        //             write!(f, "{:?} -> {:?}", key, value)?;
        //         } else {
        //             write!(f, "{:?} -> {:?}, ", key, value)?;
        //         }
        //         //index += 1;
        //     }
        //     write!(f, "]")
        // }
    }
}

#[derive(Debug, PartialEq)]
enum Kont {
    Mt,
    ExprStmtK(Rc<Kont>),
    OpK(Operation, Rc<Expr>, Rc<Kont>),
    IfK(Rc<Stmt>, Option<Rc<Stmt>>, Rc<Stmt>, Rc<Kont>), // Missing successor!
    DeclK(Name, Rc<Stmt>, Rc<Kont>),
    ReturnK(Rc<Env>, Rc<Kont>),
    CallK(Rc<Env>, Rc<ParamList>, Rc<Arguments>, Rc<Kont>),
    FunK(Rc<Env>, Rc<Kont>),
    BlocK(Rc<Env>, Rc<Stmt>, Rc<Kont>),
    AssignK(Rc<Expr>, Rc<Stmt>, Rc<Kont>),
}

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

impl From<&Rc<Stmt>> for Config {
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
    pub fn next(&self, handler: &mut ProgramHandler) -> Self {
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
                            handler.successor_lookup(s.clone()), // TODO: Successor function
                            Rc::clone(&self.k),
                        )),
                    },
                    DeclD(_, id, expr) => Self {
                        c: match expr {
                            Some(expr) => AstExpr(Rc::clone(expr)),
                            None => AstExpr(Rc::new(Val(Rc::new(UnitV)))),
                        },
                        e: Rc::clone(&self.e),
                        s: Rc::clone(&self.s),
                        k: Rc::new(DeclK(
                            id.clone(), // TODO should be copy?
                            handler.successor_lookup(s.clone()),
                            Rc::clone(&self.k),
                        )),
                    },
                    Decl(id) => Self {
                        /* Introduce variable into environment */
                        c: AstExpr(Rc::new(Val(Rc::new(UnitV)))),
                        e: {
                            let mut new_env = (*self.e).clone();
                            new_env.0.insert(id.clone(), handler.get_address());
                            Rc::new(new_env)
                        },
                        s: self.s.clone(),
                        k: Rc::new(DeclK(
                            id.clone(),
                            handler.successor_lookup(s.clone()),
                            self.k.clone(),
                        )),
                    },
                    Assign(id, expr) => Self {
                        c: AstExpr(expr.clone()),
                        e: self.e.clone(),
                        s: self.s.clone(),
                        k: Rc::new(AssignK(
                            id.clone(),
                            handler.successor_lookup(s.clone()),
                            self.k.clone(),
                        )),
                    },
                    //              Return(expr) => {
                    //              	let mut k = Rc::clone(&self.k);
                    //              	while let BlockK(_, _, inner_k) = k.as_ref() {
                    //              		k = Rc::clone(inner_k);
                    //              	}
                    //              	if let ReturnK(_, _, _) = k.as_ref() {
                    // Self {
                    // 	c: AstExpr(Rc::clone(expr)),
                    // 	e: Rc::clone(&self.e),
                    // 	s: Rc::clone(&self.s),
                    // 	k: Rc::clone(&k),
                    // }
                    //              	} else {
                    //              		panic!()
                    //              	}
                    //              }
                    Return(expr) => match self.k.as_ref() {
                        BlocK(_, _, k) => Self {
                            c: AstExpr(expr.clone()),
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
                        _ => panic!("Found some other Kont"),
                    },
                    Block(stmts) => Self {
                        c: AstStmt(stmts.get(0).unwrap().clone()),
                        e: self.e.clone(),
                        s: self.s.clone(),
                        k: Rc::new(BlocK(
                            self.e.clone(),
                            handler.successor_lookup(s.clone()),
                            self.k.clone(),
                        )),
                    },
                    Break => todo!(),
                    Goto(_) => todo!(),
                    Continue => todo!(),
                    _ => todo!(),
                }
            }
            AstExpr(e) => match e.as_ref() {
                BinaryOp(l, op, r) => {
                    if let (Val(l), Val(r)) = (l.as_ref(), r.as_ref()) {
                        self.new_c(AstExpr(Rc::new(Val(Rc::new(op.call(l, r))))))
                    } else {
                        self.no_change_map(
                            AstExpr(Rc::clone(l)),
                            OpK(*op, Rc::clone(r), Rc::clone(&self.k)),
                        )
                    }
                }
                Array(exprs) => {
                    // Get the first address
                    let addr = handler.get_address();
                    // Build the array handler value
                    let array_ref = Value::ArrayV(exprs.len(), addr);
                    // Get new store
                    let mut new_store = (*self.s).clone();
                    // new_store.0.insert(addr.clone(), v1.clone());
                    for expr in exprs.iter() {
                        if let Val(v) = expr {
                            new_store.0.insert(handler.get_address(), v.clone());
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
                Val(v) => self.invoke_kont(v, handler),
                Call(_, _) => todo!(),
                _ => todo!(),
            },
        }
    }
    fn invoke_kont(&self, v1: &Rc<Value>, handler: &mut ProgramHandler) -> Config {
        match self.k.as_ref() {
            OpK(op, expr, k) => {
                // Is the expression a value?
                match expr.as_ref() {
                    Val(v2) => Self {
                        c: AstExpr(Rc::new(Val(Rc::new(op.call(v2, v1))))),
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
            DeclK(id, succ, k) => {
                // Get new address
                let addr = handler.get_address();

                Self {
                    c: AstStmt(Rc::clone(succ)),
                    e: {
                        let mut new_env = (*self.e).clone();
                        new_env.0.insert(id.clone(), addr.clone());
                        Rc::new(new_env)
                    },
                    s: {
                        let mut new_store = (*self.s).clone();
                        new_store.0.insert(addr.clone(), Rc::clone(v1));
                        Rc::new(new_store)
                    },
                    k: Rc::clone(k),
                }
            }
            ReturnK(env, k) => Self {
                c: AstExpr(Rc::new(Expr::Val(v1.clone()))),
                e: env.clone(),
                s: self.s.clone(),
                k: k.clone(),
            },
            AssignK(id, succ, k) => {
                let addr: &Address = match id.as_ref() {
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
                        new_store.0.insert(addr.clone(), v1.clone());
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
