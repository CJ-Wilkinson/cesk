use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value {
    IntV(i32),
    BoolV(bool),
    UnitV,
    // ArrayV(Vec<Value>), TODO: This
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Expr {
    Val(Rc<Value>),
    Neg(Rc<Expr>),
    BinaryOp(Rc<Expr>, Operation, Rc<Expr>),
    Var(Name),
    Call(Name, ParamList),
    Array(Vec<Expr>),
    Index(Name, Rc<Expr>),
    Deref(Rc<Expr>),
    Ref(Name),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Type {
    IntT,
    BoolT,
    UnitT,
    ArrayT(Box<Type>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stmt {
    If(Rc<Expr>, Rc<Stmt>, Option<Rc<Stmt>>),
    Assign(Rc<Expr>, Rc<Expr>),
    ExprStmt(Rc<Expr>),
    Decl(Type, Name, Option<Rc<Expr>>),
    Return(Rc<Expr>),
    Block(Rc<Vec<Stmt>>),
    Break,
    Goto(Rc<Stmt>),
    Continue,
}

#[derive(Debug, Clone)]
pub struct Arguments(pub Vec<Expr>);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParamList(pub Vec<(Type, Name)>);

#[derive(Debug, Clone)]
pub struct Fun {
    pub name: Name,
    pub args: Arguments,
    pub body: Stmt,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub funs: HashMap<Name, Fun>,
}
