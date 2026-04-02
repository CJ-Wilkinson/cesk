use std::collections::BTreeMap;
use std::iter::Iterator;
use std::rc::Rc;

/*
Name ::= [a-zA-z][a-zA-Z0-9_]*
*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name(pub String);

/*
Fun ::= int-lit | true | false | '()'
*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value {
    IntV(i32),
    BoolV(bool),
    UnitV,
    // TODO: Arrays
}

/*
Operation ::= '+' | '*' | '-' | '/' | '%' | '==' | '!=' | '<' | '>' | '<=' | '>='
*/
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
            use Value::*;
            match self {
                Self::Add => IntV(lhs + rhs),
                Self::Sub => IntV(lhs - rhs),
                Self::Mult => IntV(lhs * rhs),
                Self::Div => IntV(lhs / rhs),
                Self::Rem => IntV(lhs % rhs),

                Self::Eq => BoolV(lhs == rhs),
                Self::Neq => BoolV(lhs != rhs),
                Self::Lt => BoolV(lhs < rhs),
                Self::Gt => BoolV(lhs > rhs),
                Self::Lte => BoolV(lhs <= rhs),
                Self::Gte => BoolV(lhs >= rhs),
            }
        } else {
            panic!("Type mismatch for operation: {:?}", self)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Expr {
    /*
    Expr ::= <Val>
    */
    Val(Rc<Value>),
    /*
    Expr ::= '-' <Expr>
    */
    Neg(Rc<Expr>),
    /*
    Expr ::= <Expr> <Operation> <Expr>
    */
    BinaryOp(Rc<Expr>, Operation, Rc<Expr>),
    /*
    Expr ::= <Name>
    */
    Var(Name),
    /*
    Expr ::= <Name> <ParamList>
    */
    Call(Name, Arguments),
    Array(Vec<Expr>),
    Index(Name, Rc<Expr>),
    Deref(Rc<Expr>),
    Ref(Name),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Type {
    /*
    Type ::= 'int'
    */
    IntT,
    /*
    Type ::= 'bool'
    */
    BoolT,
    /*
    Type ::= '()'
    */
    UnitT,
    /*
    Type ::= '?'
    */
    ArrayT(Rc<Type>),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stmt {
    /*
    Declaration that will be desugared
    DeclD ::= <Type> <Name> ('=' <Expr>)? ';'
    */
    DeclD(Type, Name, Option<Rc<Expr>>),
    /*
    For loop that will be desugared.
    If ::= 'for' '(' <Expr> ';' <Expr> ';' <Expr> ')' <Stmt>
    */
    ForD(Option<Rc<Expr>>, Rc<Expr>, Option<Rc<Expr>>, Rc<Stmt>),
    /*
    While ::= 'while' '(' <Expr> ')' <Stmt>
    */
    WhileD(Rc<Expr>, Rc<Stmt>),

    // Rules used in CESK
    /*
    The <If> statement will contain a conditional <Expr>, a true
    branch <Stmt> and an optional false branch <Stmt>.
    If ::= 'if' '(' <Expr> ')' <Stmt> ('else' <Stmt>)?
    */
    If(Rc<Expr>, Rc<Stmt>, Option<Rc<Stmt>>),
    /*
    The <Assign> will contain a assignment location (can be a variable or member of array) of <Expr>
    and the thing to be assigned <Expr>.
    */
    Assign(Rc<Expr>, Rc<Expr>),
    /*
    The <ExprStmt> will only contain some <Expr> to be evaluated.
    ExprStmt ::= <Expr> ';'
    */
    ExprStmt(Rc<Expr>),
    /*
    The <Decl> will be what introduces a <Name> into the environment. No other information should
    be needed since type checking should have occured.

    */
    Decl(Name),
    /*
    The <Return> will always return some <Expr>. This can be some value or Unit.
    */
    Return(Rc<Expr>),
    /*
    The block will hold a vector of <Stmt>.
    Block ::= '{' <Stmt>* '}'
    */
    Block(Vec<Rc<Stmt>>),
    /*
    This <Goto> will hold its jump location <Stmt>.
    */
    Goto(Rc<Stmt>),

    While(Rc<Expr>,Rc<Stmt>),
    Continue,
    Break,
}

impl Iterator for Stmt {
    type Item = Rc<Stmt>;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Block(stmts) => {
                let ret = stmts[0].clone();
                *self = Self::Block(Vec::from(stmts[1..].as_ref()));
                Some(ret)
            }
            _ => panic!("statement has no successors"),
        }
    }
}

/*
Arguments ::= '(' <Expr>* ')'
*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Arguments(pub Vec<Expr>);

/*
ParamList ::= '(' (<Type> <Name>)* ')'
*/
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParamList(pub Vec<(Type, Name)>);

/*
Fun ::= <Type> <Name> <Arguments> <Body>
*/
#[derive(Debug, Clone)]
pub struct Fun {
    pub typ: Type,
    pub name: Name,
    pub params: ParamList,
    pub body: Stmt,
}

/*
Program ::= <Fun>+
*/
#[derive(Debug, Clone)]
pub struct Program {
    pub funs: BTreeMap<Name, Fun>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_iter() {
        use Stmt::Block;
        use Stmt::Decl;

        let dec = Rc::new(Decl(Name("x".to_string())));
        let dec2 = Rc::new(Decl(Name("y".to_string())));

        let mut bl = Block(vec![dec, dec2]);

        assert_eq!(*bl.next().unwrap().as_ref(), Decl(Name("x".to_string())));
        assert_eq!(*bl.next().unwrap().as_ref(), Decl(Name("y".to_string())));
        // println!("bl: {:?}", bl.next());
        // println!("bl: {:?}", bl.next());
    }
}
