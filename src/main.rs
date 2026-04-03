use std::io;

mod ast;
mod conf;
mod display;

mod parser;
mod visit;

// pub use visit::viz;
// pub use visit::viz::expr_to_dot;
// pub use visit::viz::dot_to_png;
// pub use visit::successor_visitor::SuccessorVisitor;

use chumsky::Parser;

//
//
// use conf::Config;
// use conf::Address;
// use conf::conf::SuccessorHandler;
//use visit::visit::traverse;
//use crate::parser::parse::exp_parser;
use crate::conf::conf::Config;
use crate::conf::ProgramHandler;
use crate::parser::parse::program_parser;
use crate::parser::parse::stmt_parser;

fn main() {
    println!("Enter program: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match program_parser().parse(&input).into_result() {
        Ok(mut program) => match program.get_entry() {
            Ok(entry) => {
                let mut sigma = Config::from(&entry);
                let mut handler = ProgramHandler::from(program);
                loop {
                    println!("{}", sigma);
                    let mut input = String::new();
                    let _ = io::stdin().read_line(&mut input);
                    sigma = sigma.next(&mut handler);
                }
            }
            Err(e) => {
                println!("{}", e);
                return;
            }
        },
        Err(e) => println!("{:?}", e),
    }
}
