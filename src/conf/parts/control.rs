use crate::ast::*;
use std::rc::Rc;
use super::address::Address;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
pub enum Control {
    AstExpr(Rc<Expr>),
    AstStmt(Rc<Stmt>),
}

impl Display for Control {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Control::AstExpr(expr) => write!(f, "{}", expr),
            Control::AstStmt(stmt) => write!(f, "{}", stmt),
        }
    }
}
