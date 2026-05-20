use super::visit::Visitor;
use crate::ast::*;
use crate::visit::visit::Traverse;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct SuccessorVisitor {
    pub map: HashMap<Rc<Stmt>, Rc<Stmt>>,
}

impl SuccessorVisitor {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn create(start: &mut Stmt) -> HashMap<Rc<Stmt>, Rc<Stmt>> {
        let mut sv = SuccessorVisitor::new();
        start.traverse(&mut sv);
        if let SuccessorVisitor { map } = sv {
            map
        } else {
            panic!()
        }
    }
}

impl Visitor for SuccessorVisitor {
    fn previsit_stmt(&mut self, node: &Stmt) {
        if let Stmt::Block { stmts } = node {
            for (current, next) in stmts.iter().zip(stmts.iter().skip(1)) {
                self.map.insert(current.clone(), next.clone()); // This is cloning the Rc?
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse::stmt_parser;
    use crate::visit::visit::Traverse;
    use chumsky::Parser;

    #[test]
    fn successor_test() {
        let mut sv = SuccessorVisitor::default();
        let input = "
			{
				int thing = 3;
				{
					int other = 4;
					other = 3 + 4;
				}
				return 10;
			}
        ";
        match stmt_parser().parse(&input).into_result() {
            Ok(ast) => {
                ast.traverse(&mut sv);
                assert_eq!(sv.map.len(), 3);
                for (key, value) in sv.map {
                    println!("{}\t->\t{}", key, value);
                }
            }
            Err(e) => assert!(false),
        }
    }
}
