use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    IntV(i32),
    BoolV(bool),
    VoidV,
    ArrayV(Vec<Value>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Val(Value),

    Neg(Box<Expr>),
    ArithBinop(Box<Expr>, ArithBinop, Box<Expr>),
    CompareBinop(Box<Expr>, CompareBinop, Box<Expr>),

    Var(Name),

    Call(Name, Vec<Expr>),
    Array(Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    IntT,
    BoolT,
    VoidT,
    ArrayT(Box<Type>),
}

/// # Statements
/// s := if | = | expression | declaration (e.g. `int x = 1;`) | return (e)? | {} | while | break
///     | continue

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Assign(Expr, Expr),
    ExprStmt(Expr),
    Decl(Type, Name, Option<Expr>),
    Return(Option<Expr>),
    Block(Vec<Stmt>, HashMap<usize, Option<usize>>),
    //Block(Vec<Stmt<'tree>>, HashMap<&'tree Stmt<'tree>, &'tree Stmt<'tree>>),
    While(Expr, Box<Stmt>),
    Break,
    Continue,
}

impl<'tree> Stmt {
    pub fn make_block(stmts: Vec<Stmt>) -> Self {
        let mut m = HashMap::new();
        for i in 0..stmts.len() {
            if i+1 >= stmts.len() {
                m.insert(i, None);
                break;
            }
            m.insert(i, Some(i+1));
        }
        Self::Block(stmts, m)
        //Self::Block(stmts, HashMap::new())
        //Self::Block(stmts, stmts.iter().zip(stmts.iter().skip(1)).collect())
    }
}

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
    use super::*;

    #[test]
    fn make_block() {
        let block = vec![
            Stmt::Decl(Type::IntT, Name("x".to_string()), None),
            Stmt::Decl(Type::IntT, Name("y".to_string()), None),
        ];
        let node = Stmt::make_block(block);
        println!("Block Node: {:?}", node);
        match node {
            Stmt::Block(s, m) => {
                assert!(m[&0usize]==Some(1usize));
                assert!(m[&1usize]==None);
                assert!(s.len() == 2);
            }
            _ => assert!(false)
        }
    }
}
