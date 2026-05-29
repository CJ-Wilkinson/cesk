use chumsky::{Parser, error::Rich, extra};
use rust_cesk::ast::Program;
use rust_cesk::parser::lexer::lexer::lex;
use rust_cesk::parser::lexer::token::Token;
use rust_cesk::parser::parse::parse;
use std::fs;
use std::path::PathBuf;

pub type ParseError<'src> = extra::Err<Rich<'src, Token>>;

pub fn tokens(src: &str) -> Vec<Token> {
    lex(src)
}

pub fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

pub fn fixture_src(name: &str) -> String {
    fs::read_to_string(fixture_path(name)).unwrap()
}

pub fn fixture_tokens(name: &str) -> Vec<Token> {
    tokens(&fixture_src(name))
}

pub fn parse_fixture(name: &str) -> Program {
    parse(&fixture_tokens(name)).unwrap()
}

pub fn parse_result<'src, O, P>(
    parser: P,
    tokens: &'src [Token],
) -> Result<O, Vec<Rich<'src, Token>>>
where
    P: Parser<'src, &'src [Token], O, ParseError<'src>>,
{
    parser.parse(tokens).into_result()
}
