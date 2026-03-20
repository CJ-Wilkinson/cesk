use std::io;

pub mod ast;
pub mod conf;
pub mod display;

pub mod parser;
pub mod visit;

use chumsky::Parser;

use crate::parser::parse::program_parser;

fn main() {
    // println!("{}", &std::env::args().nth(1).unwrap());
    // parse::parse(&std::env::args().nth(1).unwrap());
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match program_parser().parse(&input).into_result() {
        Ok(ast) => println!("{:?}", ast),
        Err(e) => println!("{:?}", e),
    }
}
