use chumsky::prelude::*;

use crate::{
    ast::*,
    parser::{common::program_parser, lexer::token::Token},
};



pub fn parser<'src>() -> impl Parser<'src, &'src [Token], Program, ParseError<'src>> {
    program_parser()
}

pub fn parse(tokens: &'_ [Token]) -> Result<Program, Vec<Rich<'_, Token>>> {
    parser().parse(tokens).into_result()
}
