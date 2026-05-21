use crate::ast::*;
use crate::parser::lexer::token::{Token, TokenLexeme, TokenTag};
use TokenLexeme::*;
use TokenTag::*;

use std::rc::Rc;

use chumsky::error::Rich;
use chumsky::pratt::{infix, left, none, prefix};
use chumsky::prelude::any;
use chumsky::prelude::{choice, recursive};
use chumsky::{IterParser, Parser, extra};

use chumsky::prelude::end;
use std::collections::BTreeMap;

pub type ParseError<'src> = extra::Err<Rich<'src, Token>>;

// common signature for the parsers
//-> impl Parser<'src, &'src [Token], (), ParseError<'src>> + Clone

pub fn expect_tag<'src>(
    expected: TokenTag,
) -> impl Parser<'src, &'src [Token], (), ParseError<'src>> + Clone {
    any().try_map(move |tok: Token, span| {
        if tok.kind() == expected {
            Ok(())
        } else {
            Err(Rich::custom(span, format!("Expected token {:?}", expected)))
        }
    })
}

pub fn name_parser<'src>() -> impl Parser<'src, &'src [Token], ast::Name, ParseError<'src>> + Clone
{
    any().try_map(|tok: Token, span| match tok.kind() {
        NAME => match tok.lexeme() {
            Some(TokenLexeme::Name(s)) => Ok(s.to_string()),
            _ => return Err(Rich::custom(span, "Name missing string")),
        },
        _ => return Err(Rich::custom(span, "Expected Name")),
    })
}

pub fn scalar_type_parser<'src>() -> impl Parser<'src, &'src [Token], Type, ParseError<'src>> + Clone
{
    any().try_map(|tok: Token, span| {
        let node = match tok.kind() {
            BOOL => Type::BoolT,
            INT => Type::IntT,
            UNIT => Type::UnitT,
            _ => return Err(Rich::custom(span, "Expected ScalarType")),
        };

        Ok(node)
    })
}

pub fn type_parser<'src>() -> impl Parser<'src, &'src [Token], Type, ParseError<'src>> + Clone {
    let array_suffix = expect_tag(LSQUARE)
        .then_ignore(expect_tag(RSQUARE))
        .repeated()
        .count();

    scalar_type_parser()
        .then(array_suffix)
        .map(|(base, array_depth)| (0..array_depth).fold(base, |ty, _| Type::ArrayT(Rc::new(ty))))
}

pub fn literal_parser<'src>() -> impl Parser<'src, &'src [Token], Expr, ParseError<'src>> + Clone {
    bool_int_literal_parser().or(unit_literal_parser())
}

pub fn bool_int_literal_parser<'src>()
-> impl Parser<'src, &'src [Token], Expr, ParseError<'src>> + Clone {
    any().try_map(|tok: Token, span| {
        let lit = match tok.kind() {
            INTLIT => match tok.lexeme() {
                Some(Int(n)) => Value::IntV((*n).try_into().unwrap()), // ! brittle, fix later??
                _ => return Err(Rich::custom(span, "INTLIT missing integer lexeme")),
            },
            TRUE => Value::BoolV(true),
            FALSE => Value::BoolV(false),
            _ => return Err(Rich::custom(span, "Expected Literal")),
        };

        Ok(Expr::val(lit.into()))
    })
}

pub fn unit_literal_parser<'src>()
-> impl Parser<'src, &'src [Token], Expr, ParseError<'src>> + Clone {
    expect_tag(LPAREN)
        .then_ignore(expect_tag(RPAREN))
        .map(|_| Expr::val(Rc::new(Value::UnitV)))
}

pub fn param_parser<'src>() -> impl Parser<'src, &'src [Token], Param, ParseError<'src>> + Clone {
    type_parser()
        .then(name_parser())
        .map(|(typ, name)| Param { typ, name })
}

pub fn params_parser<'src>() -> impl Parser<'src, &'src [Token], ParamList, ParseError<'src>> + Clone
{
    expect_tag(LPAREN)
        .ignore_then(param_list_parser().or_not())
        .then_ignore(expect_tag(RPAREN))
        .map(|params| params.unwrap_or_else(|| ParamList { params: Vec::new() }))
}

pub fn param_list_parser<'src>()
-> impl Parser<'src, &'src [Token], ParamList, ParseError<'src>> + Clone {
    param_parser()
        .separated_by(expect_tag(COMMA))
        .at_least(1)
        .collect::<Vec<_>>()
        .map(|params| ParamList { params })
}

/*

pub fn assign_op_parser<'src>() -> impl Parser<'src, &'src [Token], Operation, ParseError<'src>> + Clone{
    any().try_map(|tok: Token|{
        let node = match tok.kind(){
            EQ => Operation::Assign

                };
                Ok(node)
    })
}*/

pub fn statement_parser<'src>() -> impl Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone
{
    recursive(|stmt| {
        for_d_stmt_parser(stmt.clone())
            .or(if_stmt_parser(stmt.clone()))
            .or(while_stmt_parser(stmt.clone()))
            .or(assign_stmt_parser(stmt.clone()))
            .or(decl_d_stmt_parser(stmt.clone()))
            .or(return_stmt_parser(stmt.clone()))
            .or(block_stmt_parser(stmt.clone()))
            .or(continue_stmt_parser(stmt.clone()))
            .or(break_stmt_parser(stmt.clone()))
            .or(expression_parser()
                .then(expect_tag(SEMICOLON))
                .map(|(expr, _)| Stmt::expr_stmt(expr)))
    })
}

/*

where
    P: Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone,*/
pub fn for_d_stmt_parser<'src, P>(
    stmt: P,
) -> impl Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone
where
    P: Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone,
{
    expect_tag(FOR)
        .ignore_then(expect_tag(LPAREN))
        .ignore_then(expression_parser().or_not())
        .then_ignore(expect_tag(SEMICOLON))
        .then(expression_parser())
        .then_ignore(expect_tag(SEMICOLON))
        .then(expression_parser().or_not())
        .then_ignore(expect_tag(RPAREN))
        .then(stmt)
        .map(|(((init, condition), update), body)| {
            Stmt::for_d(init, condition, update, Rc::new(body))
        })
}

pub fn if_stmt_parser<'src, P>(
    stmt: P,
) -> impl Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone
where
    P: Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone,
{
    expect_tag(IF)
        .ignore_then(expect_tag(LPAREN))
        .ignore_then(expression_parser())
        .then_ignore(expect_tag(RPAREN))
        .then(stmt.clone())
        .then(expect_tag(ELSE).ignore_then(stmt.or_not()))
        .map(|((condition, then_branch), else_branch)| {
            Stmt::if_(condition, then_branch.into(), else_branch.map(Rc::new))
        })
}

pub fn while_stmt_parser<'src, P>(
    stmt: P,
) -> impl Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone
where
    P: Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone,
{
    expect_tag(WHILE)
        .then_ignore(expect_tag(LPAREN))
        .ignore_then(expression_parser())
        .then_ignore(expect_tag(RPAREN))
        .then(stmt.clone())
        .map(|(condition, body)| Stmt::while_(condition.into(), body.into()))
}

pub fn decl_d_stmt_parser<'src, P>(
    _stmt: P,
) -> impl Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone
where
    P: Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone,
{
    type_parser()
        .then(name_parser())
        .then(expect_tag(EQ).ignore_then(expression_parser()).or_not()) // ?? do we want to support other assignment operations in the future? Trivial to add them at this point
        .then_ignore(expect_tag(SEMICOLON))
        .map(|((typ, name), expr)| Stmt::decl(typ, name, expr))
}

pub fn assign_target_parser<'src>()
-> impl Parser<'src, &'src [Token], Rc<Expr>, ParseError<'src>> + Clone {
    name_parser()
        .then(
            expect_tag(LSQUARE)
                .ignore_then(expression_parser())
                .then_ignore(expect_tag(RSQUARE))
                .or_not(),
        )
        .map(|(name, index)| match index {
            Some(index) => Rc::new(Expr::index(name, index)),
            None => Rc::new(Expr::var(name)),
        })
}

pub fn assign_stmt_parser<'src, P>(
    _stmt: P,
) -> impl Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone
where
    P: Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone,
{
    assign_target_parser()
        .then_ignore(expect_tag(EQ))
        .then(expression_parser())
        .then_ignore(expect_tag(SEMICOLON))
        .map(|(lhs, rhs)| Stmt::assign(lhs, rhs))
}

pub fn return_stmt_parser<'src, P>(
    // ! Values are required to be returned currently
    _stmt: P,
) -> impl Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone
where
    P: Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone,
{
    expect_tag(RETURN)
        .ignore_then(expression_parser())
        .then_ignore(expect_tag(SEMICOLON))
        .map(|value| Stmt::return_(value))
}

pub fn block_stmt_parser<'src, P>(
    stmt: P,
) -> impl Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone
where
    P: Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone,
{
    expect_tag(LCURLY)
        .ignore_then(stmt.repeated().collect::<Vec<_>>())
        .then_ignore(expect_tag(RCURLY))
        .map(|stmts| Stmt::block(stmts.into_iter().map(Rc::new).collect()))
}

pub fn continue_stmt_parser<'src, P>(
    _stmt: P,
) -> impl Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone {
    expect_tag(CONTINUE)
        .ignore_then(expect_tag(SEMICOLON))
        .map(|_| Stmt::continue_())
}

pub fn break_stmt_parser<'src, P>(
    _stmt: P,
) -> impl Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone
where
    P: Parser<'src, &'src [Token], Stmt, ParseError<'src>> + Clone,
{
    expect_tag(BREAK)
        .ignore_then(expect_tag(SEMICOLON))
        .map(|_| Stmt::break_())
}

/*
Expressions
*/

enum AtomicSuffix {
    Call(Arguments),
    Index(Rc<Expr>),
}

/*
highest:
  unary ! and -
  * / %
  + -
  < > <= >=
  == !=
lowest
*/
pub fn expression_parser<'src>()
-> impl Parser<'src, &'src [Token], Rc<Expr>, ParseError<'src>> + Clone {
    recursive(|expr| {
        let atom = atom_expr_parser(expr.clone());

        atom.pratt((
            prefix(5, unary_op_parser(), |op, rhs: Rc<Expr>, _| {
                Rc::new(Expr::unary_op(op, rhs))
            }),
            infix(
                left(4),
                mul_div_op_parser(),
                |lhs: Rc<Expr>, op, rhs: Rc<Expr>, _| Rc::new(Expr::binary_op(lhs, op, rhs)),
            ),
            infix(
                left(3),
                add_sub_op_parser(),
                |lhs: Rc<Expr>, op, rhs: Rc<Expr>, _| Rc::new(Expr::binary_op(lhs, op, rhs)),
            ),
            infix(
                none(2),
                relational_op_parser(),
                |lhs: Rc<Expr>, op, rhs: Rc<Expr>, _| Rc::new(Expr::binary_op(lhs, op, rhs)),
            ),
            infix(
                none(1),
                equality_op_parser(),
                |lhs: Rc<Expr>, op, rhs: Rc<Expr>, _| Rc::new(Expr::binary_op(lhs, op, rhs)),
            ),
        ))
    })
}

pub fn arguments_parser<'src, P>(
    expr: P,
) -> impl Parser<'src, &'src [Token], Arguments, ParseError<'src>> + Clone
where
    P: Parser<'src, &'src [Token], Rc<Expr>, ParseError<'src>> + Clone,
{
    expect_tag(LPAREN)
        .ignore_then(
            expr.clone()
                .separated_by(expect_tag(COMMA))
                .collect::<Vec<_>>(),
        )
        .then_ignore(expect_tag(RPAREN))
        .map(|args| Arguments { args })
}

pub fn atom_expr_parser<'src, P>(
    expr: P,
) -> impl Parser<'src, &'src [Token], Rc<Expr>, ParseError<'src>> + Clone
where
    P: Parser<'src, &'src [Token], Rc<Expr>, ParseError<'src>> + Clone + 'src,
{
    let literal = literal_parser().map(Rc::new);

    let name_like = name_parser()
        .then(
            choice((
                arguments_parser(expr.clone()).map(AtomicSuffix::Call),
                expect_tag(LSQUARE)
                    .ignore_then(expr.clone())
                    .then_ignore(expect_tag(RSQUARE))
                    .map(AtomicSuffix::Index),
            ))
            .or_not(),
        )
        .map(|(name, suffix)| match suffix {
            Some(AtomicSuffix::Call(args)) => Rc::new(Expr::call_name(name, args)),
            Some(AtomicSuffix::Index(index)) => Rc::new(Expr::index(name, index)),
            None => Rc::new(Expr::var(name)),
        });

    let array_literal = expect_tag(LSQUARE)
        .ignore_then(
            expr.clone()
                .separated_by(expect_tag(COMMA))
                .collect::<Vec<_>>(),
        )
        .then_ignore(expect_tag(RSQUARE))
        .map(|elements| Rc::new(Expr::array(elements)));

    let grouped = expect_tag(LPAREN)
        .ignore_then(expr.clone())
        .then_ignore(expect_tag(RPAREN));

    choice((literal, name_like, array_literal, grouped))
}

pub fn unary_op_parser<'src>()
-> impl Parser<'src, &'src [Token], UOperation, ParseError<'src>> + Clone {
    choice((
        expect_tag(NOT).to(UOperation::Not),
        expect_tag(MINUS).to(UOperation::Neg),
    ))
}

pub fn mul_div_op_parser<'src>()
-> impl Parser<'src, &'src [Token], Operation, ParseError<'src>> + Clone {
    choice((
        expect_tag(TIMES).to(Operation::Mult),
        expect_tag(DIVIDE).to(Operation::Div),
        expect_tag(MOD).to(Operation::Rem),
    ))
}

pub fn add_sub_op_parser<'src>()
-> impl Parser<'src, &'src [Token], Operation, ParseError<'src>> + Clone {
    choice((
        expect_tag(PLUS).to(Operation::Add),
        expect_tag(MINUS).to(Operation::Sub),
    ))
}

pub fn relational_op_parser<'src>()
-> impl Parser<'src, &'src [Token], Operation, ParseError<'src>> + Clone {
    choice((
        expect_tag(LT).to(Operation::Lt),
        expect_tag(GT).to(Operation::Gt),
        expect_tag(LEQ).to(Operation::Lte),
        expect_tag(GEQ).to(Operation::Gte),
    ))
}

pub fn equality_op_parser<'src>()
-> impl Parser<'src, &'src [Token], Operation, ParseError<'src>> + Clone {
    choice((
        expect_tag(EQEQ).to(Operation::Eq),
        expect_tag(NOTEQ).to(Operation::Neq),
    ))
}

pub fn fun_parser<'src>() -> impl Parser<'src, &'src [Token], Fun, ParseError<'src>> + Clone {
    type_parser()
        .then(name_parser())
        .then(params_parser())
        .then(block_stmt_parser(statement_parser()))
        .map(|(((typ, name), params), body)| Fun {
            typ,
            name,
            params: Rc::new(params),
            body: Rc::new(body),
        })
}

pub fn main_fun_parser<'src>() -> impl Parser<'src, &'src [Token], Fun, ParseError<'src>> + Clone {
    expect_tag(UNIT)
        .ignore_then(expect_tag(MAIN))
        .ignore_then(expect_tag(LPAREN))
        .then_ignore(expect_tag(RPAREN))
        .then(block_stmt_parser(statement_parser()))
        .map(|(_, body)| Fun {
            typ: Type::UnitT,
            name: "main".to_string(),
            params: Rc::new(ParamList { params: Vec::new() }),
            body: Rc::new(body),
        })
}

pub fn program_parser<'src>() -> impl Parser<'src, &'src [Token], Program, ParseError<'src>> + Clone
{
    fun_parser()
        .repeated()
        .collect::<Vec<_>>()
        .then(main_fun_parser())
        .then(fun_parser().repeated().collect::<Vec<_>>())
        .then_ignore(end())
        .map(|((before_main, main_fun), after_main)| {
            let mut funs = BTreeMap::new();

            for fun in before_main {
                funs.insert(fun.name.clone(), fun);
            }

            funs.insert(main_fun.name.clone(), main_fun);

            for fun in after_main {
                funs.insert(fun.name.clone(), fun);
            }

            Program { funs }
        })
}
