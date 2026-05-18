use crate::ast::*;
use crate::conf::parts::address::Address;

use std::fmt::{Display, Error, Formatter, write};

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::IntV(x) => write!(f, "{}", x),
            Self::BoolV(b) => write!(f, "{}", b),
            Self::UnitV => write!(f, "()"),
            Self::ArrayV(size, _) => write!(f, "Array(size: {})", size),
            Self::AddrV(addr) => write!(f, "Addr({})", addr),
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

// impl Display for Name {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
//         write!(f, "{}", self.0)
//     }
// }

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
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

impl Display for UOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Neg => write!(f, "-"),
            Self::Not => write!(f, "!"),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Val(v) => write!(f, "{}", v),
            Self::Neg(ex) => write!(f, "{}", ex),
            Self::BinaryOp(e1, op, e2) => write!(f, "{} {} {}", e1, op, e2),
            Self::Var(n) => write!(f, "{}", n),
            Self::CallRef(fun, args) => write!(f, "{} ({:?})", fun, args),
            Self::CallName(n, args ) => write!(f, "{} ({:?})", n, args),
            Self::Array(elems) => write!(f, "[{:?}]", elems),
            Self::Index(n, ex) => write!(f, "{}[{}]", n, ex),
            Self::UnaryOp(op, ex) => write!(f, "{}{}", op, ex),
            // Self::Deref(ex) => write!(f, "*{}", ex),  // ? Not needed?
            // Self::Ref(n) => write!(f, "&{}", n),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use Type::*;
        match self {
            IntT => write!(f, "int"),
            BoolT => write!(f, "int"),
            UnitT => write!(f, "int"),
            ArrayT(t) => write!(f, "{}[]", t),
        }
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::If(tr, bl, fal) => match fal {
                Some(fal) => write!(f, "if({}){{ {} }} else{{ {} }}", tr, bl, fal),
                None => write!(f, "if({}) {}", tr, bl),
            },
            Self::Assign(var, val) => write!(f, "{} = {};", var, val),
            Self::ExprStmt(ex) => write!(f, "{};", ex),
            //Self::DeclD(t, n, val) => match val {
                //Some(v) => write!(f, "{} {} = {};", t, n, v),
                //None => write!(f, "{} {};", t, n),
            //},
            Self::Decl(n) => {
                write!(f, "{}", n)
            }
            Self::Return(ex) => {
                write!(f, "return {}", ex)
            }
            Self::Block(v) => {
                //write!(f, "{{ {:?} }}", btree.keys().clone())
                write!(f, "{{")?;
                for stmt in v {
                    write!(f, "{}", stmt)?;
                }
                write!(f, "}}")
            }
            Self::Break => write!(f, "break;"),
            Self::Continue => write!(f, "continue;"),
            _ => write!(f, ""),
        }
    }
}

impl Display for ast::Fun {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}({:?}){{ {} }}", self.name, self.params, self.body)
    }
}
