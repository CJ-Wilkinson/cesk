use std::collections::BTreeMap;
//use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name(pub String);

//#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
//pub struct Ref<'tree>(pub &'tree Stmt);
//
//impl<'tree> Ref<'tree> {
//    fn as_usize(&self) -> usize {
//        self.0 as *const Stmt as usize
//    }
//}

//impl<'tree> Hash for Ref<'tree> {
//    fn hash<H>(&self, hasher: &mut H)
//    where
//        H: Hasher,
//    {
//        hasher.write_usize(self.as_usize());
//    }
//}
//
//impl<'tree> PartialEq for Ref<'tree> {
//    fn eq(&self, other: &Ref<'tree>) -> bool {
//        self.as_usize() == other.as_usize()
//    }
//}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value {
    IntV(i32),
    BoolV(bool),
    VoidV,
    ArrayV(Vec<Value>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArithBinop {
    Add,
    Mult,
    Sub,
    Div,
    Rem,
}

impl ArithBinop {
    pub fn call(&self, lhs: i32, rhs: i32) -> i32 {
        match self {
            Self::Add => lhs + rhs,
            Self::Sub => lhs - rhs,
            Self::Mult => lhs * rhs,
            Self::Div => lhs / rhs,
            Self::Rem => lhs % rhs,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CompareBinop {
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
}

impl CompareBinop {
    pub fn call(&self, lhs: i32, rhs: i32) -> bool {
        match self {
            Self::Eq => lhs == rhs,
            Self::Neq => lhs != rhs,
            Self::Lt => lhs < rhs,
            Self::Gt => lhs > rhs,
            Self::Lte => lhs <= rhs,
            Self::Gte => lhs >= rhs,
        }
    }
}

/// # Expressions
/// e := i32 | - (negative) | + | * | - (subtraction) | / | % | == | != | < |  <= | >= | label
///     | fn call | []
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Expr {
    Val(Value),

    Neg(Box<Expr>),
    ArithBinop(Box<Expr>, ArithBinop, Box<Expr>),
    CompareBinop(Box<Expr>, CompareBinop, Box<Expr>),

    Var(Name),

    Call(Name, Vec<Expr>),
    Array(Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Type {
    IntT,
    BoolT,
    VoidT,
    ArrayT(Box<Type>),
}

/// # Statements
/// s := if | = | expression | declaration (e.g. `int x = 1;`) | return (e)? | {} | while | break
///     | continue

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stmt {
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Assign(Expr, Expr),
    ExprStmt(Expr),
    Decl(Type, Name, Option<Expr>),
    Return(Option<Expr>),
    Block(BTreeMap<Rc<Stmt>, Option<Rc<Stmt>>>),
    While(Expr, Box<Stmt>),
    Break,
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
    pub rtype: Type,
    pub name: Name,
    pub args: Vec<(Type, Name)>,
    pub body: Stmt,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub funs: Vec<Fun>,
}

#[cfg(test)]
mod tests {
    /*
    use super::*;

    #[test]
    fn make_block() {
        let contents = vec![
            Stmt {
                contents: StmtContents::Decl(Type::IntT, Name("x".to_string()), None),
                label: None,
                successor: None,
                parent: None,
            },
            Stmt {
                contents: StmtContents::Decl(Type::IntT, Name("y".to_string()), None),
                label: None,
                successor: None,
                parent: None,
            },
        ];
        let node = Stmt {
            label: None,
            contents: StmtContents::Block(contents),
            successor: None,
            parent: None,
        };
        println!("Block Node: {:?}", node);
        match node {
            Stmt {
                contents: StmtContents::Block(s, m),
                ..
            } => {
                assert!(m[&0usize] == Some(1usize));
                assert!(m[&1usize] == None);
                assert!(s.len() == 2);
            }
            _ => assert!(false),
        }
    }
    */
}
