use std::collections::BTreeMap;
use std::rc::Rc;
use std::iter::Iterator;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value {
    IntV(i32),
    BoolV(bool),
    UnitV,
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

    Call(Name, ParamList),
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
    Return(Rc<Expr>),
    Block(BTreeMap<Rc<Stmt>, Option<Rc<Stmt>>>),
    Break,
    Goto(Rc<Stmt>),
    Continue,
}

impl Iterator for Stmt {
    type Item = (Rc<Stmt>, Option<Rc<Stmt>>);
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Block(tree) => {
                if let Some((key, val)) = tree.iter().next() {
                    Some((key.clone(), val.clone()))
                        
                } else { 
                    panic!()
                }
            }
            _ => panic!("statement has no successors")
        }
    }
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

#[derive(Debug, Clone)]
pub struct Arguments(pub Vec<Expr>);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParamList(pub Vec<Name>);

/// # Function
/// a function consists of a return type, a name, a list of args, and a body statement
#[derive(Debug, Clone)]
pub struct Fun {
    pub name: Name,
    pub args: Arguments,
    pub body: Stmt,
}

impl Iterator for Fun {
    type Item = Stmt;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.body.clone())     // do we want to clone this?
    }
}

// TODO: Change to HashMap

#[derive(Debug, Clone)]
pub struct Program {
    pub funs: Vec<Fun>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn block_iter() {
        use Stmt::Block as Block;
        use Stmt::Decl as Decl;

        let temp_map: BTreeMap<Rc<Stmt>, Option<Rc<Stmt>>> = BTreeMap::from([
            (Rc::new(Decl(Name("x".to_string()), None)), None),
            // (Rc::new(Decl(Name("y".to_string()), None)), None),
        ]);

        let mut bl = Block(temp_map);

        let succ = bl.next().unwrap();
        println!("{:?}", succ.0);
        // assert!(succ.0.as_ref() == Decl(Name("x".to_string()), None));
        // assert_eq!(succ.0.as_ref(), Decl(Name("x".to_string()), None));
    }
}
