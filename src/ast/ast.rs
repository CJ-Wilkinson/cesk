use std::collections::HashMap;
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
    ArrayT(Box<Type>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    Block(Rc<Vec<Stmt>>),
    /*
	This <Goto> will hold its jump location <Stmt>.
    */
    Goto(Rc<Stmt>),
    Continue,
    Break,
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
    pub funs: HashMap<Name, Fun>,
}
