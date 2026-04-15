use crate::ast::*;
use std::rc::Rc;
use super::environment::Env;

#[derive(Debug, PartialEq)]
pub enum Kont {
    Mt,
    ExprStmtK(Rc<Stmt>, Rc<Kont>),
    OpK(Operation, Rc<Expr>, Rc<Kont>),
    IfK(Rc<Stmt>, Option<Rc<Stmt>>, Rc<Stmt>, Rc<Kont>),
    DeclK(Name, Rc<Stmt>, Rc<Kont>),
    ReturnK(Rc<Env>, Rc<Kont>),
    CallK(Rc<Env>,Rc<Fun>, Rc<ParamList>, Rc<Arguments>, Rc<Kont>),
    FunK(Rc<Env>, Rc<Kont>),
    BlocK(Rc<Env>, Rc<Stmt>, Rc<Kont>), // ! I would like to rename this
    AssignK(Rc<Expr>, Rc<Stmt>, Rc<Kont>),
    WhileK(Rc<Env>, Rc<Expr>, Rc<Stmt>, Rc<Stmt>, Rc<Kont>),
    IdK(Rc<Expr>, Rc<Stmt>, Rc<Kont>), // ! Get rid of this
    LvalK(Rc<Value>, Rc<Stmt>, Rc<Kont>)
}
