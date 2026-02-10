use crate::ast::{Expr, Fun, Name, Program, Stmt, Type, Value};
use chumsky::prelude::{choice, just, recursive, text, IterParser, Parser};

pub fn typ_parser<'src>() -> impl Parser<'src, &'src str, Type> {
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

pub fn exp_parser<'src>() -> impl Parser<'src, &'src str, Expr> {
    let int = text::int(10)
        .map(|s: &str| Value::IntV(s.parse().unwrap()))
        .padded();
    let true_v = text::ascii::keyword("true").padded().to(Value::BoolV(true));
    let false_v = text::ascii::keyword("false")
        .padded()
        .to(Value::BoolV(false));
    let bool_v = choice((true_v, false_v));

    let value = choice((int, bool_v));
    let expr = recursive(|expr| {
        choice((
            value.map(|v| Expr::Val(v)),
            just('-')
                .ignore_then(expr.clone())
                .map(|e| Expr::Neg(Box::new(e))),
            ident_parser()
                .then_ignore(just('('))
                .then(expr.clone().separated_by(just(',')).collect::<Vec<_>>())
                .then_ignore(just(')'))
                .map(|(name, args)| Expr::Call(name, args)),
            ident_parser().map(|n| Expr::Var(n)),
            just('[')
                .ignore_then(expr.clone().separated_by(just(',')).collect::<Vec<_>>())
                .then_ignore(just(']'))
                .map(|exprs| Expr::Array(exprs)),
        ))
    });
    expr
}

pub fn stmt_parser<'src, 'tree>() -> impl Parser<'src, &'src str, Stmt<'tree>> {
    // TODO other Stmt variants
    choice((
        exp_parser()
            .then_ignore(just(';'))
            .map(|e| Stmt::ExprStmt(e)),
        exp_parser()
            .then_ignore(just('=').padded())
            .then(exp_parser())
            .map(|(lhs, rhs)| Stmt::Assign(lhs, rhs)),
    ))
}

pub fn fun_parser<'src, 'tree>() -> impl Parser<'src, &'src str, Fun<'tree>> {
    typ_parser()
        .then(ident_parser())
        .then_ignore(just('(').padded())
        .then(
            (typ_parser().padded().then(ident_parser().padded()))
                .separated_by(just(','))
                .collect::<Vec<_>>(),
        )
        .then_ignore(just(')').padded())
        .then_ignore(just('{').padded())
        .then(
            stmt_parser()
                .repeated()
                .collect::<Vec<_>>()
                .map(|b| Stmt::make_block(b)),
        )
        .then_ignore(just('}').padded())
        .map(|(((rtype, name), args), body)| Fun {
            rtype,
            name,
            args,
            body,
        })
}

pub fn program_parser<'src, 'tree>() -> impl Parser<'src, &'src str, Program<'tree>> {
    fun_parser()
        .repeated()
        .collect::<Vec<_>>()
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
