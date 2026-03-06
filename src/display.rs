use std::fmt::{Display, Formatter, Error};
use crate::ast::*;

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_> ) -> Result<(), Error> {
        match self {
            Self::IntV(x) => write!(f, "{}", x),
            Self::BoolV(b) => write!(f, "{}", b),
            Self::VoidV => write!(f, "void"),

            // Self::ArrayV(vec) => {
            //     let mut s = String::new();
            //     for elem in vec {
            //         s += elem.trim().parse();
            //     }
            //     write!(f, "{:?}", vec)
            // }
        }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_> ) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_> ) -> Result<(), Error> {
        match self {
            Self::Add => write!(f, "+"),
            Self::Mult => write!(f, "*"),
            Self::Sub => write!(f, "-"),
            Self::Div => write!(f, "/"),
            Self::Rem => write!(f, "%"),
            Self::Eq => write!(f, "=="),
            Self::Neq => write!(f, "!="),
            Self::Lt => write!(f, "<"),
            Self::Gt => write!(f, ">"),
            Self::Lte => write!(f, "<="),
            Self::Gte => write!(f, ">="),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_> ) -> Result<(), Error> {
        match self {
            Self::Val(v) => write!(f, "{}", v),
            Self::Neg(ex) => write!(f, "{}", ex),
            Self::Op(e1, op, e2) => write!(f, "{} {} {}", e1, op, e2),
            Self::Var(n) => write!(f, "{}", n),
            Self::Call(n, args) => write!(f, "{} ({:?})", n, args),
            Self::Array(elems) => write!(f, "[{:?}]", elems),
            Self::Index(n, ex) => write!(f, "{}[{}]", n, ex),
            Self::Deref(ex) => write!(f, "*{}", ex),
            Self::Ref(n) => write!(f, "&{}", n),
        }
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_> ) -> Result<(), Error> {
        match self {
            Self::If(tr, bl, fal) => {
                match fal {
                    Some(fal) => write!(f, "if({}) {} else {}", tr, bl, fal),
                    None => write!(f, "if({}) {}", tr, bl),
                }
            },
            Self::Assign(var, val) => write!(f, "{} = {}", var, val),
            Self::ExprStmt(ex) => write!(f, "{}", ex),
            Self::Decl(n, val) => {
                match val {
                    Some(v) => write!(f, "{} = {}", n, v),
                    None => write!(f, "{}", n),
                }
            },
            Self::Return(ex) => {
                match ex {
                    Some(e) => write!(f, "return {}", e),
                    None => write!(f, "return"),
                }
            },
            Self::Block(btree) => {
                write!(f, "{{ {:?} }}", btree.keys().clone())
            },
            Self::Break => write!(f, "break"),
            Self::Goto(st) => write!(f, "goto: {}", st),
            Self::Continue => write!(f, "continue"),
        }
    }
}

impl Display for ast::Fun {
    fn fmt(&self, f: &mut Formatter<'_> ) -> Result<(), Error> {
        write!(f, "{}({:?}){{ {} }}", self.name, self.args, self.body)
    }
}
