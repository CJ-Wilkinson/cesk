use std::io;
use std::io::Read;

mod ast;
mod conf;
mod display;

mod parser;
mod visit;

// pub use visit::viz;
// pub use visit::viz::expr_to_dot;
// pub use visit::viz::dot_to_png;
// pub use visit::successor_visitor::SuccessorVisitor;

// use chumsky::Parser;

//
//
// use conf::Config;
// use conf::Address;
// use conf::conf::SuccessorHandler;
//use visit::visit::traverse;
//use crate::parser::parse::exp_parser;
use crate::conf::ProgramHandler;
use crate::conf::conf::Config;
use crate::parser::lexer::lexer::lex;
use crate::parser::parse::parse;
use clap::Parser;


#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    step_through: bool,

    #[arg(short, long)]
    debug: bool,

    #[arg(short, long)]
    file: Option<String>,
}


fn main() {

    let args = Args::parse();
    let mut input = Vec::new();

    // read from file if filename is given in command line
    if let Some(file) = args.file {
        input = std::fs::read(file).unwrap();
    } else {
        println!("Enter program: ");

        io::stdin()
            .read_to_end(&mut input)
            .expect("Failed to read line");

    }
    let src = std::str::from_utf8(&input).unwrap_or("");

    let tokens = lex(src);

    match parse(&tokens) {
        Ok(mut program) => match program.get_entry() {
            Ok(entry) => {
                let mut sigma = Config::from(&entry);
                let mut handler = ProgramHandler::from(program);

                loop {
                    println!("{}", sigma);

                    if args.step_through == true {
                        let mut input = String::new();
                        let _ = io::stdin().read_line(&mut input);
                    }

                    sigma = sigma.next(&mut handler);
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        },

        Err(errors) => {
            for error in errors {
                println!("{:?}", error);
            }
        }
    }
}
