use super::ast::*;
use std::convert::From;
use std::rc::Rc;

// Arguments
impl Arguments {
    pub fn slice_ref(&self) -> &[Rc<Expr>] {
        &self.args
    }
}
impl From<&[Rc<Expr>]> for Arguments {
    fn from(value: &[Rc<Expr>]) -> Self {
        Self {
            args: value.to_vec(),
        }
    }
}
impl ParamList {
    pub fn slice_ref(&self) -> &[(Type, Name)] {
        &self.params
    }
}
impl From<&[(Type, Name)]> for ParamList {
    fn from(value: &[(Type, Name)]) -> Self {
        Self {
            params: value.to_vec(),
        }
    }
}
