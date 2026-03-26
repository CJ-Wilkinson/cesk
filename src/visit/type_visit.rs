use super::visit::Visitor;
use crate::ast::*;
use crate::visit::visit::Traverse;
use std::collections::HashMap;
use std::rc::Rc;

struct TypeCheck {
	scope_stack: Vec<HashMap<Name, Type>>,
	type_stack: Vec<Type>,
	error_stack: Vec<String>,
}

impl TypeCheck {
	pub fn new() -> Self {
		Self {
			scope_stack: Vec::new(),
			type_stack: Vec::new(),
		}
	}
	pub fn get(&mut self, name: Name) -> Type {
		// TODO if you can't find symbol, push error onto error stack and return unknown
	}
	pub fn insert_symbol(&mut self, name: Name, typ: Type) {
		// TODO check for redeclaration
		self.scope_stack.first().insert(name, typ);
	}
}

impl Visitor for TypeCheck {

    fn previsit_name(&mut self, _node: &Name) {}
    fn previsit_value(&mut self, _node: &Value) {}
    fn previsit_operation(&mut self, _node: &Operation) {}
    fn previsit_expr(&mut self, _node: &Expr) {}
    fn previsit_type(&mut self, _node: &Type) {}
    fn previsit_stmt(&mut self, node: &Stmt) {
	    /*
			Only block can introduce a new scope
	    */
    	match node {
    		// Create a new scope and push onto the stack
    		Stmt::Block(_) => self.scope_stack.push(HashMap::new()),
    		// Bind symbol to scope
    		Stmt::Decl(typ, name) => self.insert_symbol(name.clone(), typ.clone()),
    		_ => (),
    		}
    	}
    fn previsit_arguments(&mut self, _node: &Arguments) {}
    fn previsit_paramlist(&mut self, _node: &ParamList) {}
    fn previsit_fun(&mut self, node: &Fun) {
    	/*
		Function has a scope with parameters
    	*/
    	match node {
    		Fun{typ: _, name: _, params, body: _} => {
    			for (typ, name) in &params.0 {
    				self.insert_symbol(name.clone(), typ.clone());
    			}
    		}
    	}
    }
    fn previsit_program(&mut self, _node: &Program) {}

    fn postvisit_name(&mut self, _node: &Name) {}
    fn postvisit_value(&mut self, _node: &Value) {}
    fn postvisit_operation(&mut self, _node: &Operation) {}
    fn postvisit_expr(&mut self, node: &Expr) {
    	match node {
    		Expr::BinaryOp(_, _, _) => {
    			let (lhs, rhs) = (self.type_stack.pop(), self.type_stack.pop());
    		}
    		_ => (),
    	}
    }
    fn postvisit_type(&mut self, node: &Type) {
    	self.type_stack.push(node.clone());
    }
    fn postvisit_stmt(&mut self, _node: &Stmt) {}
    fn postvisit_arguments(&mut self, _node: &Arguments) {}
    fn postvisit_paramlist(&mut self, _node: &ParamList) {}
    fn postvisit_fun(&mut self, _node: &Fun) {}
    fn postvisit_program(&mut self, _node: &Program) {}
}
