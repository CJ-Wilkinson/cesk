use std::io;

pub mod ast;
pub mod conf;
pub mod display;

pub mod parser;
pub mod visit;

pub use visit::viz;
pub use visit::viz::expr_to_dot;
pub use visit::viz::dot_to_png;

use chumsky::Parser;


use conf::Config;

use crate::parser::parse::exp_parser;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match exp_parser().parse(&input).into_result() {
        Ok(ast) => {
        	// Run the visitor
			let _ = dot_to_png(&expr_to_dot(&ast), "thing");
			
        	let mut sigma = Config::from(ast);
        	loop {
        		println!("{}", sigma);
        		sigma = sigma.next();
        	}
        },
        Err(e) => println!("{:?}", e),
    }
}
