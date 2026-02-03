use crate::ast::{Expr, Fun, Stmt, Type, Value};
use chumsky::prelude::Parser;

/*
pub fn parser<'src>() -> impl Parser<'src, '&src str, Program<'src> {
todo!()
}
*/

pub fn parse(filename: &str) {
    let src = std::fs::read_to_string(filename).unwrap();
    println!("{src}");
    let stmts: Vec<_> = src.split_whitespace().collect();
    println!("stmts: {:?}", stmts);
}
