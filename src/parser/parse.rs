use crate::ast::{Expr, Fun, Name, Program, Stmt, Type, Value};
use chumsky::prelude::{choice, just, text, IterParser, Parser};

pub fn parser<'src>() -> impl Parser<'src, &'src str, Program<'src>> {
    let typ = choice((
        text::ascii::keyword("int").padded().to(Type::IntT),
        text::ascii::keyword("bool").padded().to(Type::BoolT),
    ));

    let int = text::int(10)
        .map(|s: &str| Expr::Val(Value::IntV(s.parse().unwrap())))
        .padded();
    let true_v = text::ascii::keyword("true")
        .padded()
        .to(Expr::Val(Value::BoolV(true)));
    let false_v = text::ascii::keyword("false")
        .padded()
        .to(Expr::Val(Value::BoolV(false)));
    let bool_v = choice((true_v, false_v));

    let ident = text::ascii::ident().padded().map(|name| Name { name });

    let value = choice((int, bool_v));
    let expr = value;

    let stmt = expr
        .then_ignore(just(';'))
        .map(|e| Stmt::ExprStmt(Box::new(e)));

    let fun = typ
        .clone()
        .then(ident)
        .then_ignore(just('(').padded())
        .then(
            (typ.padded().then(ident.padded()))
                .separated_by(just(','))
                .collect::<Vec<_>>(),
        )
        .then_ignore(just(')').padded())
        .then_ignore(just('{').padded())
        .then(stmt.repeated().collect::<Vec<_>>().map(|b| Stmt::Block(b)))
        .then_ignore(just('}').padded())
        .map(|(((rtype, name), args), body)| Fun {
            rtype,
            name,
            args,
            body,
        });
    fun.repeated()
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

    #[test]
    fn one_fun() -> Result<(), ()> {
        match parser().parse("void test() {}").into_result() {
            Ok(_) => Ok(()),
            Err(errs) => {
                errs.into_iter().for_each(|e| println!("Error: {e}"));
                Err(())
            }
        }
    }
}
