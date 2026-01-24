use std;

pub mod parser;

pub use crate::parser::parse;

fn main() {
    parse::parse(&std::env::args().nth(1).unwrap());
}
