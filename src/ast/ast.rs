use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value {
    IntV(i32),
    BoolV(bool),
    VoidV,
    // ArrayV(Vec<Value>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Operation {
    Add,
    Mult,
    Sub,
    Div,
    Rem,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
}

impl Operation {
    pub fn call(&self, lhs: &Value, rhs: &Value) -> Value {
        if let (Value::IntV(lhs), Value::IntV(rhs)) = (lhs, rhs) {
            match self {
                Self::Add => Value::IntV(lhs + rhs),
                Self::Sub => Value::IntV(lhs - rhs),
                Self::Mult => Value::IntV(lhs * rhs),
                Self::Div => Value::IntV(lhs / rhs),
                Self::Rem => Value::IntV(lhs % rhs),

                Self::Eq => Value::BoolV(lhs == rhs),
                Self::Neq => Value::BoolV(lhs != rhs),
                Self::Lt => Value::BoolV(lhs < rhs),
                Self::Gt => Value::BoolV(lhs > rhs),
                Self::Lte => Value::BoolV(lhs <= rhs),
                Self::Gte => Value::BoolV(lhs >= rhs),
            }
        } else {
            panic!("Type mismatch for operation: {:?}", self)
        }
    }
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub enum CompareBinop {
//     Eq,
//     Neq,
//     Lt,
//     Gt,
//     Lte,
//     Gte,
// }

// impl CompareBinop {
//     pub fn call(&self, lhs: i32, rhs: i32) -> bool {
//         match self {
//             Self::Eq => lhs == rhs,
//             Self::Neq => lhs != rhs,
//             Self::Lt => lhs < rhs,
//             Self::Gt => lhs > rhs,
//             Self::Lte => lhs <= rhs,
//             Self::Gte => lhs >= rhs,
//         }
//     }
// }

/// # Expressions
/// e := i32 | - (negative) | + | * | - (subtraction) | / | % | == | != | < |  <= | >= | label
///     | fn call | []
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Expr {
    Val(Rc<Value>),

    Neg(Rc<Expr>),
    Op(Rc<Expr>, Operation, Rc<Expr>),
    //CompareOp(Rc<Expr>, CompareBinop, Rc<Expr>),
    Var(Name),

    Call(Name, Vec<Expr>),
    Array(Vec<Expr>),
    Index(Name, Rc<Expr>),
    Deref(Rc<Expr>),
    Ref(Name),
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub enum Type {
//     IntT,
//     BoolT,
//     VoidT,
//     ArrayT,
// }

/// # Statements
/// s := if | = | expression | declaration (e.g. `int x = 1;`) | return (e)? | {} | while | break
///     | continue

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stmt {
    If(Rc<Expr>, Rc<Stmt>, Option<Rc<Stmt>>),
    Assign(Rc<Expr>, Rc<Expr>),
    ExprStmt(Rc<Expr>),
    Decl(Name, Option<Rc<Expr>>),
    Return(Option<Rc<Expr>>),
    Block(BTreeMap<Rc<Stmt>, Option<Rc<Stmt>>>),
    Break,
    Goto(Rc<Stmt>),
    Continue,
}

/*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stmt {
    pub label: Option<Name>,
    pub contents: StmtContents<'tree>,
    pub successor: Option<&'tree Stmt>,
    pub parent: Option<&'tree Stmt>,
}
*/

/// # Function
/// a function consists of a return type, a name, a list of args, and a body statement
#[derive(Debug, Clone)]
pub struct Fun {
    pub name: Name,
    pub args: Vec<Name>,
    pub body: Stmt,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub funs: Vec<Fun>,
}
