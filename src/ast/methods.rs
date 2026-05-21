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

// ParamList
impl ParamList {
    pub fn slice_ref(&self) -> &[Param] {
        &self.params
    }
}

impl From<&[Param]> for ParamList {
    fn from(value: &[Param]) -> Self {
        Self {
            params: value.to_vec(),
        }
    }
}
