use crate::ast::*;
use std::rc::Rc;
use super::environment::Env;

#[derive(Debug, PartialEq)]
pub enum Kont {
    Mt,
    ExprStmtK(Rc<Stmt>, Rc<Kont>),
    OpLK(Operation, Rc<Expr>, Rc<Kont>),
    OpRK(Operation, Rc<Value>, Rc<Kont>),
    IfK(Rc<Stmt>, Option<Rc<Stmt>>, Rc<Stmt>, Rc<Kont>),
    ReturnK(Rc<Env>, Rc<Kont>),
    CallK(Rc<Env>,Rc<Fun>, Rc<ParamList>, Rc<Arguments>, Rc<Kont>),
    FunK(Rc<Env>, Rc<Kont>),
    BlockK(Rc<Env>, Rc<Stmt>, Rc<Kont>), // ! I would like to rename this
    AssignK(Rc<Expr>, Rc<Stmt>, Rc<Kont>),
    WhileK(Rc<Env>, Rc<Expr>, Rc<Stmt>, Rc<Stmt>, Rc<Kont>),
    LvalK(Rc<Value>, Rc<Stmt>, Rc<Kont>),
    IndexK(Rc<Name>, Rc<Kont>)
}
