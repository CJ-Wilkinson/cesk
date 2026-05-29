mod common;

use chumsky::{Parser, prelude::end};
use rust_cesk::ast::{Expr, Operation, Stmt, Type, UOperation, Value};
use rust_cesk::parser::common::{
    expression_parser, statement_parser, type_parser, unit_literal_parser,
};
use rust_cesk::parser::parse::parse;

fn parse_all<'src, O, P>(parser: P, tokens: &'src [rust_cesk::parser::lexer::token::Token]) -> O
where
    P: Parser<'src, &'src [rust_cesk::parser::lexer::token::Token], O, common::ParseError<'src>>,
{
    common::parse_result(parser.then_ignore(end()), tokens).unwrap()
}

#[test]
fn parses_unit_type_and_unit_literal_separately() {
    let tokens = common::tokens("unit");
    assert_eq!(parse_all(type_parser(), &tokens), Type::UnitT);

    let tokens = common::tokens("()");
    let expr = parse_all(unit_literal_parser(), &tokens);
    assert_eq!(
        expr,
        Expr::Val {
            value: Value::UnitV.into()
        }
    );
}

#[test]
fn parses_array_types() {
    let tokens = common::tokens("int[][]");
    assert_eq!(
        parse_all(type_parser(), &tokens),
        Type::ArrayT(Type::ArrayT(Type::IntT.into()).into())
    );
}

#[test]
fn parses_expression_precedence() {
    let tokens = common::tokens("1 + 2 * 3");
    let expr = parse_all(expression_parser(), &tokens);

    assert_eq!(
        *expr,
        Expr::BinaryOp {
            lhs: Expr::Val {
                value: Value::IntV(1).into()
            }
            .into(),
            op: Operation::Add,
            rhs: Expr::BinaryOp {
                lhs: Expr::Val {
                    value: Value::IntV(2).into()
                }
                .into(),
                op: Operation::Mult,
                rhs: Expr::Val {
                    value: Value::IntV(3).into()
                }
                .into(),
            }
            .into(),
        }
    );
}

#[test]
fn parses_unary_expression() {
    let tokens = common::tokens("!false");
    let expr = parse_all(expression_parser(), &tokens);

    assert_eq!(
        *expr,
        Expr::UnaryOp {
            op: UOperation::Not,
            expr: Expr::Val {
                value: Value::BoolV(false).into()
            }
            .into(),
        }
    );
}

#[test]
fn parses_declaration_statement_with_initializer() {
    let tokens = common::tokens("int x = 42;");
    let stmt = parse_all(statement_parser(), &tokens);

    assert_eq!(
        stmt,
        Stmt::Decl {
            typ: Type::IntT,
            name: "x".to_string(),
            expr: Some(
                Expr::Val {
                    value: Value::IntV(42).into()
                }
                .into()
            ),
        }
    );
}

#[test]
fn parses_minimal_program_with_main() {
    let tokens = common::tokens("int main() { return 2; }");
    let program = parse(&tokens).unwrap();

    let main = program.funs.get("main").unwrap();
    assert_eq!(main.typ, Type::IntT);
    assert_eq!(main.params.params, Vec::new());
    assert_eq!(
        *main.body,
        Stmt::Block {
            stmts: vec![
                Stmt::Return {
                    expr: Expr::Val {
                        value: Value::IntV(2).into()
                    }
                    .into()
                }
                .into()
            ]
        }
    );
}
