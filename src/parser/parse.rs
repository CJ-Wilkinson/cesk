use std::collections::BTreeMap;
use std::rc::Rc;

use crate::ast::*;
// use chumsky::error::EmptyErr;
use chumsky::pratt::{infix, left, none, prefix};
use chumsky::prelude::{choice, just, recursive, text, IterParser, Parser};

pub fn typ_parser<'src>() -> impl Parser<'src, &'src str, Type> + Clone {
    (choice((
        text::ascii::keyword("int").padded().to(Type::IntT),
        text::ascii::keyword("bool").padded().to(Type::BoolT),
        text::ascii::keyword("unit").padded().to(Type::UnitT),
    ))
    .then(
        just('[')
            .padded()
            .then_ignore(just(']').padded())
            .repeated()
            .count(),
    ))
    .map(|(base, mut array_depth)| {
        let mut result: Type = base;
        while array_depth > 0 {
            result = Type::ArrayT(Rc::new(result));
            array_depth -= 1;
        }
        result
    })
}

pub fn ident_parser<'src>() -> impl Parser<'src, &'src str, Name> + Clone {
    text::ascii::ident()
        .padded()
        .map(|name: &'src str| name.to_string())
}

pub fn add_sub_parser<'src>() -> impl Parser<'src, &'src str, Operation> + Clone {
    choice((
        just('+').padded().to(Operation::Add),
        just('-').padded().to(Operation::Sub),
    ))
}

pub fn mult_div_parser<'src>() -> impl Parser<'src, &'src str, Operation> + Clone {
    choice((
        just('*').padded().to(Operation::Mult),
        just('/').padded().to(Operation::Div),
        just('%').padded().to(Operation::Rem),
    ))
}

pub fn arith_binop_parser<'src>() -> impl Parser<'src, &'src str, Operation> + Clone {
    choice((add_sub_parser(), mult_div_parser()))
}

pub fn compare_binop_parser<'src>() -> impl Parser<'src, &'src str, Operation> + Clone {
    choice((
        just('<').padded().to(Operation::Lt),
        just('>').padded().to(Operation::Gt),
        text::ascii::keyword("<=").padded().to(Operation::Lte),
        text::ascii::keyword(">=").padded().to(Operation::Gte),
        text::ascii::keyword("==").padded().to(Operation::Eq),
        text::ascii::keyword("!=").padded().to(Operation::Neq),
    ))
}

fn atomic_exp_parser<'src, P>(expr: P) -> impl Parser<'src, &'src str, Rc<Expr>> + Clone
where
    P: Parser<'src, &'src str, Rc<Expr>> + Clone + 'src,
{
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
        value.map(|e| Rc::new(Expr::Val(Rc::new(e)))),
        ident_parser()
            .then(
                just('(')
                    .padded()
                    .ignore_then(expr.clone().separated_by(just(',')).collect::<Vec<_>>())
                    .then_ignore(just(')').padded())
                    .or_not(),
            )
            .map(|(n, args)| match args {
                Some(a) => Rc::new(Expr::CallName(n, Arguments(a))),
                None => Rc::new(Expr::Var(n)),
            }),
        just('(')
            .padded()
            .ignore_then(expr.clone())
            .then_ignore(just(')').padded()),
        ident_parser()
            .then(
                just('[')
                    .padded()
                    .ignore_then(expr)
                    .then_ignore(just(']').padded())
                    .or_not(),
            )
            .map(|(e, i)| match i {
                Some(index) => Rc::new(Expr::Index(e, index)),
                None => Rc::new(Expr::Var(e)),
            }),
    ))
}

pub fn exp_parser<'src>() -> impl Parser<'src, &'src str, Rc<Expr>> + Clone {
    fn make_arith(lhs: Rc<Expr>, op: Operation, rhs: Rc<Expr>) -> Rc<Expr> {
        Rc::new(Expr::BinaryOp(lhs.clone(), op, rhs.clone()))
    }

    recursive(|expr: chumsky::prelude::Recursive<dyn Parser<'_, &str, Rc<Expr>>>| {
        let atom = atomic_exp_parser(expr.clone());

        choice((
            atom.pratt((
                infix(none(1), compare_binop_parser(), |l: Rc<Expr>, op: Operation, r: Rc<Expr>, _| {
                    Rc::new(Expr::BinaryOp(l, op, r))
                }),
                infix(left(2), add_sub_parser(), |l: Rc<Expr>, o: Operation, r: Rc<Expr>, _| make_arith(l, o, r)),
                infix(left(3), mult_div_parser(), |l: Rc<Expr>, o: Operation, r: Rc<Expr>, _| make_arith(l, o, r)),
                prefix(4, just('-').padded(), |_, e: Rc<Expr>, _| Rc::new(Expr::Neg(e))),
            )),
            just('[')
                .padded()
                .ignore_then(expr.clone().separated_by(just(',')).collect::<Vec<_>>())
                .then_ignore(just(']'))
                .map(|v| Rc::new(Expr::Array(v))),
        ))
    })
}

fn block_parser<'src, P>(stmt: P) -> impl Parser<'src, &'src str, Stmt> + Clone
where
    P: Parser<'src, &'src str, Stmt> + Clone + 'src,
{
    just('{')
        .padded()
        .ignore_then(stmt.repeated().collect::<Vec<_>>().map(|stmts| {
            Stmt::Block(
                stmts
                    .into_iter()
                    .map(|s| {
                        // let next: Option<Rc<Stmt>> = None;
                        // (Rc::new(s), next)
                        Rc::new(s)
                    })
                    .collect(),
            )
        }))
        .then_ignore(just('}').padded())
}

pub fn stmt_parser<'src>() -> impl Parser<'src, &'src str, Stmt> + Clone {
    recursive(|stmt| {
        let block = block_parser(stmt.clone());

        choice((
            exp_parser()
                .then_ignore(just('=').padded())
                .then(exp_parser())
                .then_ignore(just(';').padded())
                .map(|(lhs, rhs)| Stmt::Assign(lhs, rhs)),
            exp_parser()
                .then_ignore(just(';').padded())
                .map(|e| Stmt::ExprStmt(e)),
            block.clone(),
            text::ascii::keyword("if")
                .padded()
                .ignore_then(just('(').padded())
                .ignore_then(exp_parser())
                .then_ignore(just(')').padded())
                .then(block.clone())
                .then(
                    text::ascii::keyword("else")
                        .padded()
                        .ignore_then(block.clone())
                        .or_not(),
                )
                .map(|((cond, t), f)| Stmt::If(cond, Rc::new(t), f.map(Rc::new))),
            text::ascii::keyword("while")
                .padded()
                .ignore_then(just('(').padded())
                .ignore_then(exp_parser())
                .then_ignore(just(')').padded())
                .then(block.clone())
                .map(|(cond, t)| Stmt::While(cond, Rc::new(t))),
            text::ascii::keyword("return")
                .padded()
                .ignore_then(exp_parser())
                .then_ignore(just(';').padded())
                .map(|e| Stmt::Return(e)),
            typ_parser()
                .then(ident_parser())
                .then(just('=').padded().ignore_then(exp_parser()).or_not())
                .then_ignore(just(';').padded())
                .map(|((typ, name), init)| {
                    if let Some(init) = init {
                        panic!("old decld");
                        //Stmt::DeclD(typ, name, Some(init))
                    } else {
                        Stmt::Decl(name)
                    }
                }),
        ))
    })
}

pub fn fun_parser<'src, P>(stmt: P) -> impl Parser<'src, &'src str, Fun> + Clone
where
    P: Parser<'src, &'src str, Stmt> + Clone + 'src,
{
    typ_parser()
        .then(ident_parser())
        .then_ignore(just('(').padded())
        .then(
            (typ_parser().padded().then(ident_parser().padded()))
                .separated_by(just(','))
                .collect::<Vec<_>>(),
        )
        .then_ignore(just(')').padded())
        .then(stmt)
        .map(|(((typ, name), args), body)| Fun {
            typ,
            name,
            params: Rc::new(ParamList(args)),
            body: Rc::new(body),
        })
}

pub fn program_parser<'src>() -> impl Parser<'src, &'src str, Program> {
    let stmt = stmt_parser();

    fun_parser(stmt)
        .repeated()
        .collect::<Vec<Fun>>()
        .map(|funs| {
            let mut prog: BTreeMap<Name, Fun> = BTreeMap::new();
            for f in funs {
                prog.insert(f.name.clone(), f);
            }
            Program { funs: prog }
        })
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

    fn exp_test(exp: &str) -> Result<Rc<Expr>, ()> {
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
            Ok(Rc::new(Expr::CallName( 
                Name("f".to_string()),
                Arguments(vec![
                    Rc::new(Expr::Val(Rc::new(Value::IntV(1)))),
                    Rc::new(Expr::Val(Rc::new(Value::IntV(3))))
                ])
            )))
        )
    }

    #[test]
    fn bogus_fun_call() {
        assert!(exp_test("f(;)").is_err())
    }

    // #[test]
    // fn array_type() {
    //     assert_eq!(
    //         stmt_test("int[] x;"),
    //         Ok(Stmt::Decl(
    //             Type::ArrayT(Rc::new(Type::IntT)),
    //             Name("x".to_string()),
    //             None
    //         ))
    //     )
    // }

    #[test]
    fn assignment() {
        if let Ok(Stmt::Assign(lhs, rhs)) = stmt_test("x = 3;") {
            assert_eq!(lhs, Rc::new(Expr::Var(Name("x".to_string()))));
            assert_eq!(rhs, Rc::new(Expr::Val(Rc::new(Value::IntV(3)))));
        }
    }

    #[test]
    fn identifier() {
        assert_eq!(exp_test("x"), Ok(Rc::new(Expr::Var(Name("x".to_string())))))
    }

    #[test]
    fn number() {
        assert_eq!(exp_test("3"), Ok(Rc::new(Expr::Val(Rc::new(Value::IntV(3))))))
    }

    #[test]
    fn one_fun() {
        assert!(program_test("unit test() {}").is_ok())
    }

    #[test]
    fn one_fun_extra_char() {
        assert!(program_test("unit test() {}}").is_err())
    }

    #[test]
    fn one_fun_missing_char() {
        assert!(program_test("unit test() {").is_err())
    }
}
