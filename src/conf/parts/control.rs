use super::address::Address;
use crate::ast::*;
use std::fmt::{Display, Error, Formatter};
use std::rc::Rc;

#[derive(Debug)]
pub enum Control {
    AstExpr(Rc<Expr>),
    AstStmt(Rc<Stmt>),
}

impl Display for Control {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Control::AstExpr(expr) => write!(f, "{:?}", expr),
            Control::AstStmt(stmt) => write!(f, "{:?}", stmt),
        }
    }
}
