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
            Self::Val { value } => write!(f, "{}", value),
            //Self::Neg(ex) => write!(f, "{}", ex),
            Self::BinaryOp { lhs, op, rhs } => write!(f, "{} {} {}", lhs, op, rhs),
            Self::Var { name } => write!(f, "{}", name),
            Self::CallRef { fun, args } => write!(f, "{} ({:?})", fun, args),
            Self::CallName { callee, args } => write!(f, "{} ({:?})", callee, args),
            Self::Array { elements } => write!(f, "[{:?}]", elements),
            Self::Index { index, array } => write!(f, "{}[{}]", array, index),
            Self::UnaryOp { op, expr } => write!(f, "{}{}", op, expr),
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
            BoolT => write!(f, "bool"),
            UnitT => write!(f, "unit"),
            ArrayT(t) => write!(f, "{}[]", t),
        }
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::If {
                condition,
                then_branch,
                else_branch,
            } => match else_branch {
                Some(fal) => write!(
                    f,
                    "if({}){{ {} }} else{{ {} }}",
                    condition, then_branch, fal
                ),
                None => write!(f, "if({}) {}", condition, then_branch),
            },
            Self::Assign { lhs, rhs } => write!(f, "{} = {};", lhs, rhs),
            Self::ExprStmt { expr } => write!(f, "{};", expr),
            //Self::DeclD(t, n, val) => match val {
            //Some(v) => write!(f, "{} {} = {};", t, n, v),
            //None => write!(f, "{} {};", t, n),
            //},
            Self::Decl { typ, name, expr } => {
                if let Some(expr) = expr {
                    return write!(f, "{} {} {}", typ, name, expr);
                }
                write!(f, "{} {}", typ, name)
            }
            Self::Return { expr } => {
                write!(f, "return {}", expr)
            }
            Self::Block { stmts } => {
                //write!(f, "{{ {:?} }}", btree.keys().clone())
                write!(f, "{{")?;
                for stmt in stmts {
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
