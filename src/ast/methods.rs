use std::convert::From;
use super::ast::*;
use std::rc::Rc;

// Arguments
impl Arguments {
    pub fn slice_ref(&self) -> &[Rc<Expr>] {
        &self.0
    }
}
impl From<&[Rc<Expr>]> for Arguments{
    fn from(value: &[Rc<Expr>]) -> Self {
        Self (value.to_vec())
    }
}
impl ParamList {
    pub fn slice_ref(&self) -> &[(Type, Name)] {
        &self.0
    }
}
impl From<&[(Type, Name)]> for ParamList {
    fn from(value: &[(Type, Name)]) -> Self {
        Self (value.to_vec())
    }
}