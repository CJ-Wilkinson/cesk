use std::collections::BTreeMap;
use std::iter::Iterator;
use std::rc::Rc;

use crate::conf::parts::address::Address;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Expr {
    Val {
        value: Rc<Value>,
    },
    UnaryOp {
        op: UOperation,
        expr: Rc<Expr>,
    },
    BinaryOp {
        lhs: Rc<Expr>,
        op: Operation,
        rhs: Rc<Expr>,
    },
    Var {
        name: Name,
    },
    CallName {
        callee: Name,
        args: Arguments,
    },

    Array {
        elements: Vec<Rc<Expr>>,
    },
    Index {
        array: Name,
        index: Rc<Expr>,
    },
    #[allow(dead_code)]
    CallRef {
        fun: Rc<Fun>,
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
    ArrayT(Rc<Type>),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stmt {
    ForD {
        init: Option<Rc<Expr>>,
        condition: Rc<Expr>,
        update: Option<Rc<Expr>>,
        body: Rc<Stmt>,
    },
    If {
        condition: Rc<Expr>,
        then_branch: Rc<Stmt>,
        else_branch: Option<Rc<Stmt>>,
    },
    Assign {
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
    ExprStmt {
        expr: Rc<Expr>,
    },
    Decl {
        typ: Type,
        name: Name,
        expr: Option<Rc<Expr>>,
    },
    Return {
        expr: Rc<Expr>,
    },
    Block {
        stmts: Vec<Rc<Stmt>>,
    },
    While {
        condition: Rc<Expr>,
        body: Rc<Stmt>,
    },
    Continue,
    Break,
    //ForD(Option<Rc<Expr>>, Rc<Expr>, Option<Rc<Expr>>, Rc<Stmt>),
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
    pub args: Vec<Rc<Expr>>,
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
    pub params: Rc<ParamList>,
    pub body: Rc<Stmt>,
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
    pub fn get_entry(&mut self) -> Result<Rc<Stmt>, &str> {
        match self.funs.get("main") {
            Some(fun) => Ok(fun.body.clone()),
            None => Err("failed to get entry point"),
        }
    }
}
impl Stmt {
    pub fn for_d(
        init: Option<Rc<Expr>>,
        condition: Rc<Expr>,
        update: Option<Rc<Expr>>,
        body: Rc<Stmt>,
    ) -> Stmt {
        Stmt::ForD {
            init,
            condition,
            update,
            body,
        }
    }

    pub fn if_(condition: Rc<Expr>, then_branch: Rc<Stmt>, else_branch: Option<Rc<Stmt>>) -> Stmt {
        Stmt::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    pub fn assign(lhs: Rc<Expr>, rhs: Rc<Expr>) -> Stmt {
        Stmt::Assign { lhs, rhs }
    }

    pub fn expr_stmt(expr: Rc<Expr>) -> Stmt {
        Stmt::ExprStmt { expr }
    }

    pub fn decl(typ: Type, name: Name, expr: Option<Rc<Expr>>) -> Stmt {
        Stmt::Decl { typ, name, expr }
    }

    pub fn return_(expr: Rc<Expr>) -> Stmt {
        Stmt::Return { expr }
    }

    pub fn block(stmts: Vec<Rc<Stmt>>) -> Stmt {
        Stmt::Block { stmts }
    }

    pub fn while_(condition: Rc<Expr>, body: Rc<Stmt>) -> Stmt {
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
    pub fn val(value: Rc<Value>) -> Expr {
        Expr::Val { value }
    }

    pub fn unary_op(op: UOperation, expr: Rc<Expr>) -> Expr {
        Expr::UnaryOp { op, expr }
    }

    pub fn binary_op(lhs: Rc<Expr>, op: Operation, rhs: Rc<Expr>) -> Expr {
        Expr::BinaryOp { lhs, op, rhs }
    }

    pub fn var(name: Name) -> Expr {
        Expr::Var { name }
    }

    pub fn call_name(callee: Name, args: Arguments) -> Expr {
        Expr::CallName { callee, args }
    }

    pub fn array(elements: Vec<Rc<Expr>>) -> Expr {
        Expr::Array { elements }
    }

    pub fn index(array: Name, index: Rc<Expr>) -> Expr {
        Expr::Index { array, index }
    }

    #[allow(dead_code)]
    pub fn call_ref(fun: Rc<Fun>, args: Arguments) -> Expr {
        Expr::CallRef { fun, args }
    }
}

impl Iterator for Stmt {
    type Item = Rc<Stmt>;
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
