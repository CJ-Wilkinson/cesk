use std::rc::Rc;

use crate::ast::{ArithBinop, CompareBinop, Expr, Fun, Name, Program, Stmt, Type, Value};
// use chumsky::error::EmptyErr;
use chumsky::pratt::{infix, left, none, prefix};
use chumsky::prelude::{choice, just, recursive, text, IterParser, Parser};

pub fn typ_parser<'src>() -> impl Parser<'src, &'src str, Type> + Clone {
    choice((
        text::ascii::keyword("int").padded().to(Type::IntT),
        text::ascii::keyword("bool").padded().to(Type::BoolT),
        text::ascii::keyword("void").padded().to(Type::VoidT),
    ))
}

pub fn ident_parser<'src>() -> impl Parser<'src, &'src str, Name> + Clone {
    text::ascii::ident()
        .padded()
        .map(|name: &'src str| Name(name.to_string()))
}

pub fn add_sub_parser<'src>() -> impl Parser<'src, &'src str, ArithBinop> + Clone {
    choice((
        just('+').padded().to(ArithBinop::Add),
        just('-').padded().to(ArithBinop::Sub),
    ))
}

pub fn mult_div_parser<'src>() -> impl Parser<'src, &'src str, ArithBinop> + Clone {
    choice((
        just('*').padded().to(ArithBinop::Mult),
        just('/').padded().to(ArithBinop::Div),
        just('%').padded().to(ArithBinop::Rem),
    ))
}

pub fn arith_binop_parser<'src>() -> impl Parser<'src, &'src str, ArithBinop> + Clone {
    choice((add_sub_parser(), mult_div_parser()))
}

pub fn compare_binop_parser<'src>() -> impl Parser<'src, &'src str, CompareBinop> + Clone {
    choice((
        just('<').padded().to(CompareBinop::Lt),
        just('>').padded().to(CompareBinop::Gt),
        text::ascii::keyword("<=").padded().to(CompareBinop::Lte),
        text::ascii::keyword(">=").padded().to(CompareBinop::Gte),
        text::ascii::keyword("==").padded().to(CompareBinop::Eq),
        text::ascii::keyword("!=").padded().to(CompareBinop::Neq),
    ))
}

fn atomic_exp_parser<'src>() -> impl Parser<'src, &'src str, Expr> + Clone {
    let int = text::int(10)
        .map(|s: &str| Value::IntV(s.parse().unwrap()))
        .padded();
    let true_v = text::ascii::keyword("true").padded().to(Value::BoolV(true));
    let false_v = text::ascii::keyword("false")
        .padded()
        .to(Value::BoolV(false));
    let bool_v = choice((true_v, false_v));

    let value = choice((int, bool_v));
    choice((
        value.map(|v| Expr::Val(v)),
        ident_parser()
            .then(
                just('(')
                    .padded()
                    .ignore_then(exp_parser().separated_by(just(',')).collect::<Vec<_>>())
                    .then_ignore(just(')').padded())
                    .or_not(),
            )
            .map(|(n, args)| match args {
                Some(a) => Expr::Call(n, a),
                None => Expr::Var(n),
            }),
        just('(')
            .padded()
            .ignore_then(exp_parser())
            .then_ignore(just(')').padded()),
    ))
    .then(
        just('[')
            .padded()
            .ignore_then(exp_parser())
            .then_ignore(just(']').padded())
            .or_not(),
    )
    .map(|(e, i)| match i {
        Some(index) => Expr::Index(Box::new(e), Box::new(index)),
        None => e,
    })
}

pub fn exp_parser<'src>() -> impl Parser<'src, &'src str, Expr> + Clone {
    fn make_arith(lhs: Expr, op: ArithBinop, rhs: Expr) -> Expr {
        Expr::ArithBinop(Box::new(lhs), op, Box::new(rhs))
    }
    let expr = recursive(|expr| {
        choice((
            atomic_exp_parser().pratt((
                infix(none(1), compare_binop_parser(), |l, op, r, _| {
                    Expr::CompareBinop(Box::new(l), op, Box::new(r))
                }),
                infix(left(2), add_sub_parser(), |l, o, r, _| make_arith(l, o, r)),
                infix(left(3), mult_div_parser(), |l, o, r, _| make_arith(l, o, r)),
                prefix(4, just('-').padded(), |_, e, _| Expr::Neg(Box::new(e))),
            )),
            just('[')
                .padded()
                .ignore_then(expr.clone().separated_by(just(',')).collect::<Vec<_>>())
                .then_ignore(just(']'))
                .map(|exprs| Expr::Array(exprs)),
        ))
    });
    expr
}

pub fn block_parser<'src>() -> impl Parser<'src, &'src str, Stmt> + Clone {
    just('{')
        .padded()
        .ignore_then(stmt_parser().repeated().collect::<Vec<_>>().map(|stmts| {
            Stmt::Block(
                stmts
                    .into_iter()
                    .map(|s| {
                        let next: Option<Rc<Stmt>> = None;
                        (Rc::new(s), next)
                    })
                    .collect(),
            )
        }))
        .then_ignore(just('}').padded())
}

pub fn stmt_parser<'src>() -> impl Parser<'src, &'src str, Stmt> + Clone {
    // TODO other Stmt variants
    let stmt = recursive(|_stmt| {
        choice((
            exp_parser()
                .then_ignore(just(';'))
                .map(|e| Stmt::ExprStmt(e)),
            exp_parser()
                .then_ignore(just('=').padded())
                .then(exp_parser())
                .then_ignore(just(';').padded())
                .map(|(lhs, rhs)| Stmt::Assign(lhs, rhs)),
            block_parser(),
            text::ascii::keyword("if")
                .padded()
                .ignore_then(just('(').padded())
                .ignore_then(exp_parser())
                .then_ignore(just(')').padded())
                .then(block_parser())
                .then(
                    text::ascii::keyword("else")
                        .padded()
                        .ignore_then(block_parser())
                        .or_not(),
                )
                .map(|((cond, t), f)| Stmt::If(cond, Box::new(t), f.map(|s| Box::new(s)))),
            text::ascii::keyword("while")
                .padded()
                .ignore_then(just('(').padded())
                .ignore_then(exp_parser())
                .then_ignore(just(')').padded())
                .then(block_parser())
                .map(|(cond, t)| Stmt::While(cond, Box::new(t))),
            text::ascii::keyword("return")
                .padded()
                .ignore_then(exp_parser().or_not())
                .then_ignore(just(';').padded())
                .map(|exp| Stmt::Return(exp)),
            typ_parser()
                .then(ident_parser())
                .then(just('=').padded().ignore_then(exp_parser()).or_not())
                .then_ignore(just(';').padded())
                .map(|((typ, name), init)| Stmt::Decl(typ, name, init)),
        ))
    });
    stmt
}

pub fn fun_parser<'src>() -> impl Parser<'src, &'src str, Fun> {
    typ_parser()
        .then(ident_parser())
        .then_ignore(just('(').padded())
        .then(
            (typ_parser().padded().then(ident_parser().padded()))
                .separated_by(just(','))
                .collect::<Vec<_>>(),
        )
        .then_ignore(just(')').padded())
        .then(block_parser())
        .map(|(((rtype, name), args), body)| Fun {
            rtype,
            name,
            args,
            body,
        })
}

pub fn program_parser<'src>() -> impl Parser<'src, &'src str, Program> {
    fun_parser()
        .repeated()
        .collect::<Vec<Fun>>()
        .map(|funs| Program { funs })
}

pub fn parse(filename: &str) {
    let src = std::fs::read_to_string(filename).unwrap();
    println!("{src}");
    let stmts: Vec<_> = src.split_whitespace().collect();
    println!("stmts: {:?}", stmts);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn program_test(program_string: &str) -> Result<Program, ()> {
        program_parser()
            .parse(&program_string)
            .into_result()
            .map_err(|_| ())
    }

    fn stmt_test(stmt: &str) -> Result<Stmt, ()> {
        stmt_parser().parse(stmt).into_result().map_err(|_| ())
    }

    fn exp_test(exp: &str) -> Result<Expr, ()> {
        exp_parser().parse(exp).into_result().map_err(|_| ())
    }

    #[test]
    fn function_call_no_args() {
        assert!(exp_test("f()").is_ok())
    }

    #[test]
    fn function_call_one_arg() {
        assert!(exp_test("f(3)").is_ok())
    }

    #[test]
    fn function_call_two_arg() {
        assert_eq!(
            exp_test("f(1,3)"),
            Ok(Expr::Call(
                Name("f".to_string()),
                vec![Expr::Val(Value::IntV(1)), Expr::Val(Value::IntV(3))]
            ))
        )
    }

    #[test]
    fn bogus_fun_call() {
        assert!(exp_test("f(;)").is_err())
    }

    #[test]
    fn assignment() {
        if let Ok(Stmt::Assign(lhs, rhs)) = stmt_test("x = 3;") {
            assert_eq!(lhs, Expr::Var(Name("x".to_string())));
            assert_eq!(rhs, Expr::Val(Value::IntV(3)));
        }
    }

    #[test]
    fn identifier() {
        assert_eq!(exp_test("x"), Ok(Expr::Var(Name("x".to_string()))))
    }

    #[test]
    fn number() {
        assert_eq!(exp_test("3"), Ok(Expr::Val(Value::IntV(3))))
    }

    #[test]
    fn one_fun() {
        assert!(program_test("void test() {}").is_ok())
    }

    #[test]
    fn one_fun_extra_char() {
        assert!(program_test("void test() {}}").is_err())
    }

    #[test]
    fn one_fun_missing_char() {
        assert!(program_test("void test() {").is_err())
    }
}
