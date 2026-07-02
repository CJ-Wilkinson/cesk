use std::cmp::{Eq, Ordering, PartialEq};
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::iter::Iterator;
use std::sync::{Arc, Mutex};

use crate::conf::parts::address::Address;

use std::convert::From;

/*
Name ::= [a-zA-z][a-zA-Z0-9_]*
Fun ::= int-lit | true | false | '()'
Operation ::= '+' | '*' | '-' | '/' | '%' | '==' | '!=' | '<' | '>' | '<=' | '>='

Expr ::= <Val>
    |'-' <Expr>
    |<Expr> <Operation> <Expr>
    |<Name>
    |<Name> <ParamList>

Type ::= 'int'
    |'bool'
    |'()'
    |'?'

Arguments ::= '(' <Expr>* ')'

ParamList ::= '(' (<Type> <Name>)* ')'

Fun ::= <Type> <Name> <Arguments> <Body>

Program ::= <Fun>+



Rules used in CESK
The <If> statement will contain a conditional <Expr>, a true
branch <Stmt> and an optional false branch <Stmt>.
If ::= 'if' '(' <Expr> ')' <Stmt> ('else' <Stmt>)?

Declaration that will be desugared
DeclD ::= <Type> <Name> ('=' <Expr>)? ';'

For loop that will be desugared.
If ::= 'for' '(' <Expr> ';' <Expr> ';' <Expr> ')' <Stmt>

The <Assign> will contain a assignment location (can be a variable or member of array) of <Expr>
and the thing to be assigned <Expr>.

The <ExprStmt> will only contain some <Expr> to be evaluated.
ExprStmt ::= <Expr> ';'

The <Decl> will be what introduces a <Name> into the environment. No other information should
be needed since type checking should have occured.


The <Return> will always return some <Expr>. This can be some value or Unit.

The block will hold a vector of <Stmt>.
Block ::= '{' <Stmt>* '}'
*/

// #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub struct Name(pub String);

#[derive(Debug, Clone)]
pub struct Node<T>(Arc<Mutex<T>>);

impl<T> Node<T> {
    pub fn ptr_eq(node1: &Self, node2: &Self) -> bool {
        Arc::ptr_eq(&node1.0, &node2.0)
    }
    pub fn get_arc(&self) -> &Arc<Mutex<T>> {
        &self.0
    }
    pub fn new(inner: T) -> Self {
        Node(Arc::new(Mutex::new(inner)))
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        Node::ptr_eq(self, other)
    }
}

impl<T> Eq for Node<T> {}

impl<T> Hash for Node<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Arc::as_ptr(self.get_arc()).hash(state);
    }
}

impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        let self_ptr = Arc::as_ptr(&self.0) as usize;
        let other_ptr = Arc::as_ptr(&other.0) as usize;
        self_ptr.partial_cmp(&other_ptr)
    }
}

impl<T> Ord for Node<T> {
    fn cmp(&self, other: &Node<T>) -> Ordering {
        let self_ptr = Arc::as_ptr(&self.0) as usize;
        let other_ptr = Arc::as_ptr(&other.0) as usize;
        self_ptr.cmp(&other_ptr)
    }
}

impl<T> From<T> for Node<T> {
    fn from(inner: T) -> Self {
        Node(Arc::new(Mutex::new(inner)))
    }
}

pub type Name = String;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value {
    IntV(i32),
    BoolV(bool),
    UnitV,
    ArrayV {
        //Type, // Type of array
        size: usize,
        start_of_array: Address,
    },
    AddrV(Address),
}

impl Value {
    pub fn get_type(&self) -> Type {
        match self {
            Value::IntV(_) => Type::IntT,
            Value::BoolV(_) => Type::BoolT,
            Value::UnitV => Type::UnitT,
            Value::ArrayV { .. } => todo!(), // TODO: array values need to store their type?
            Value::AddrV(_) => Type::IntT,   // TODO: what type is an address?
        }
    }
}
// thing = [1, 2, 3]

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
                //operations
                Self::Add => IntV(lhs + rhs),
                Self::Sub => IntV(lhs - rhs),
                Self::Mult => IntV(lhs * rhs),
                Self::Div => IntV(lhs / rhs),
                Self::Rem => IntV(lhs % rhs),

                //comparisons
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UOperation {
    Neg,
    Not,
}

impl UOperation {
    pub fn call(&self, rhs: &Value) -> Value {
        match self {
            Self::Neg => {
                if let Value::IntV(rhs) = rhs {
                    return Value::IntV(-rhs);
                } else {
                    panic!("expected integer value")
                }
            }
            Self::Not => {
                if let Value::BoolV(rhs) = rhs {
                    return Value::BoolV(!rhs);
                } else {
                    panic!("Expected boolean value")
                }
            }
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub enum Expr {
    Val {
        value: Node<Value>,
    },
    UnaryOp {
        op: UOperation,
        expr: Node<Expr>,
    },
    BinaryOp {
        lhs: Node<Expr>,
        op: Operation,
        rhs: Node<Expr>,
    },
    Var {
        name: Name,
    },
    CallName {
        callee: Name,
        args: Arguments,
    },

    Array {
        elements: Vec<Node<Expr>>,
    },
    Index {
        array: Name,
        index: Node<Expr>,
    },
    #[allow(dead_code)]
    CallRef {
        fun: Node<Fun>,
        args: Arguments,
    },
    //Neg(Rc<Expr>),
    //BinaryOp(Rc<Expr>, Operation, Rc<Expr>),
    //UnaryOp(UOperation, Rc<Expr>),
    //Var(Name),
    //CallName(Name, Arguments),
    //CallRef(Rc<Fun>, Arguments), // ! Change everything over\
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Type {
    IntT,
    BoolT,
    UnitT,
    ArrayT(Node<Type>),
}

#[derive(Debug, Clone, Hash)]
pub enum Stmt {
    ForD {
        init: Option<Node<Expr>>,
        condition: Node<Expr>,
        update: Option<Node<Expr>>,
        body: Node<Stmt>,
    },
    If {
        condition: Node<Expr>,
        then_branch: Node<Stmt>,
        else_branch: Option<Node<Stmt>>,
    },
    Assign {
        lhs: Node<Expr>,
        rhs: Node<Expr>,
    },
    ExprStmt {
        expr: Node<Expr>,
    },
    Decl {
        typ: Type,
        name: Name,
        expr: Option<Node<Expr>>,
    },
    Return {
        expr: Node<Expr>,
    },
    Block {
        stmts: Vec<Node<Stmt>>,
    },
    While {
        condition: Node<Expr>,
        body: Node<Stmt>,
    },
    Continue,
    Break,
    //ForD(Option<Node<Expr>>, Rc<Expr>, Option<Rc<Expr>>, Rc<Stmt>),
    //If(Rc<Expr>, Rc<Stmt>, Option<Rc<Stmt>>),
    //DeclD(Type, Name, Option<Rc<Expr>>),
    //Assign(Rc<Expr>, Rc<Expr>),
    //ExprStmt(Rc<Expr>),
    //Decl(Name),
    //Return(Rc<Expr>),
    //Block(Vec<Rc<Stmt>>),
    //While(Rc<Expr>, Rc<Stmt>),
    //Continue,
    //Break,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Arguments {
    pub args: Vec<Node<Expr>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParamList {
    pub params: Vec<Param>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Param {
    pub typ: Type,
    pub name: Name,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Fun {
    pub typ: Type,
    pub name: Name,
    pub params: Node<ParamList>,
    pub body: Node<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub funs: BTreeMap<Name, Fun>,
}

impl Program {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            funs: BTreeMap::new(),
        }
    }
    pub fn get_entry(&mut self) -> Result<Node<Stmt>, &str> {
        match self.funs.get("main") {
            Some(fun) => Ok(fun.body.clone()),
            None => Err("failed to get entry point"),
        }
    }
}
impl Stmt {
    pub fn for_d(
        init: Option<Node<Expr>>,
        condition: Node<Expr>,
        update: Option<Node<Expr>>,
        body: Node<Stmt>,
    ) -> Stmt {
        Stmt::ForD {
            init,
            condition,
            update,
            body,
        }
    }

    pub fn if_(
        condition: Node<Expr>,
        then_branch: Node<Stmt>,
        else_branch: Option<Node<Stmt>>,
    ) -> Stmt {
        Stmt::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    pub fn assign(lhs: Node<Expr>, rhs: Node<Expr>) -> Stmt {
        Stmt::Assign { lhs, rhs }
    }

    pub fn expr_stmt(expr: Node<Expr>) -> Stmt {
        Stmt::ExprStmt { expr }
    }

    pub fn decl(typ: Type, name: Name, expr: Option<Node<Expr>>) -> Stmt {
        Stmt::Decl { typ, name, expr }
    }

    pub fn return_(expr: Node<Expr>) -> Stmt {
        Stmt::Return { expr }
    }

    pub fn block(stmts: Vec<Node<Stmt>>) -> Stmt {
        Stmt::Block { stmts }
    }

    pub fn while_(condition: Node<Expr>, body: Node<Stmt>) -> Stmt {
        Stmt::While { condition, body }
    }

    pub fn continue_() -> Stmt {
        Stmt::Continue
    }

    pub fn break_() -> Stmt {
        Stmt::Break
    }
}

impl Expr {
    pub fn val(value: Node<Value>) -> Expr {
        Expr::Val { value }
    }

    pub fn unary_op(op: UOperation, expr: Node<Expr>) -> Expr {
        Expr::UnaryOp { op, expr }
    }

    pub fn binary_op(lhs: Node<Expr>, op: Operation, rhs: Node<Expr>) -> Expr {
        Expr::BinaryOp { lhs, op, rhs }
    }

    pub fn var(name: Name) -> Expr {
        Expr::Var { name }
    }

    pub fn call_name(callee: Name, args: Arguments) -> Expr {
        Expr::CallName { callee, args }
    }

    pub fn array(elements: Vec<Node<Expr>>) -> Expr {
        Expr::Array { elements }
    }

    pub fn index(array: Name, index: Node<Expr>) -> Expr {
        Expr::Index { array, index }
    }

    #[allow(dead_code)]
    pub fn call_ref(fun: Node<Fun>, args: Arguments) -> Expr {
        Expr::CallRef { fun, args }
    }
}

impl Iterator for Stmt {
    type Item = Node<Stmt>;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Block { stmts } => {
                let ret = stmts[0].clone();
                *self = Stmt::block(Vec::from(stmts[1..].as_ref()));
                Some(ret)
            }
            _ => panic!("statement has no successors"),
        }
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn block_iter() {
        //use Stmt::Block;
        //use Stmt::Decl;

        //let dec = Rc::new(Decl(Name("x".to_string())));
        //let dec2 = Rc::new(Decl(Name("y".to_string())));

        //let mut bl = Block(vec![dec, dec2]);

        //assert_eq!(*bl.next().unwrap().as_ref(), Decl(Name("x".to_string())));
        //assert_eq!(*bl.next().unwrap().as_ref(), Decl(Name("y".to_string())));
        // println!("bl: {:?}", bl.next());
        // println!("bl: {:?}", bl.next());
    }
}
