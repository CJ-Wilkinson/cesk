use std::process::Command;
use std::{fs, io};

use crate::ast::*;
use crate::visit::visit::{Traverse, Visitor};

pub fn program_to_dot(program: &Program) -> String {
    let mut viz = GraphVizVisitor::new();
    program.traverse(&mut viz);
    viz.finish()
}

pub fn expr_to_dot(program: &Expr) -> String {
    let mut viz = GraphVizVisitor::new();
    program.traverse(&mut viz);
    viz.finish()
}

pub fn dot_to_png(dot: &str, path: &str) -> io::Result<()> {
    fs::create_dir_all("out")?;

    let dot_path = "out/ast.dot";
    fs::write(dot_path, dot)?;

    let status = Command::new("dot")
        .args(["-Tpng", dot_path, "-o", path])
        .status()?;

    if !status.success() {
        return Err(io::Error::other("graphviz dot command failed"));
    }

    Ok(())
}

pub struct GraphVizVisitor {
    next_id: usize,
    output: String,
    stack: Vec<usize>,
}

impl GraphVizVisitor {
    pub fn new() -> Self {
        let mut output = String::new();
        output.push_str("digraph AST {\n");
        output.push_str("  node [shape=box];\n");

        Self {
            next_id: 0,
            output,
            stack: Vec::new(),
        }
    }

    pub fn finish(mut self) -> String {
        self.output.push_str("}\n");
        self.output
    }

    fn enter_node(&mut self, label: impl AsRef<str> + std::fmt::Display) {
        let id = self.next_id;
        self.next_id += 1;

        //         let lbel = escape_dot_label(label.as_ref());

        self.output
            .push_str(&format!("   n{id} [label=\"{label}\"];\n"));

        if let Some(&parent) = self.stack.last() {
            self.output.push_str(&format!("   n{parent} -> n{id}; \n"));
        }

        self.stack.push(id);
    }
    fn exit_node(&mut self) {
        self.stack.pop();
    }
}

fn escape_dot_label(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

impl Visitor for GraphVizVisitor {
    fn previsit_program(&mut self, _node: &Program) {
        self.enter_node("Program");
    }
    fn previsit_name(&mut self, node: &Name) {
        self.enter_node(format!("Name({})", node));
    }

    fn previsit_value(&mut self, node: &Value) {
        let label = match node {
            Value::IntV(n) => format!("Value::IntV({n})"),
            Value::BoolV(b) => format!("Value::BoolV({b})"),
            Value::UnitV => "Value::UnitV".to_string(),
            Value::ArrayV(size, _) => format!("Array(size: {})", size),
            Value::AddrV(a) => format!("Addr({})", a),
        };
        self.enter_node(label);
    }
    fn previsit_operation(&mut self, node: &Operation) {
        self.enter_node(format!("Operation::{node:?}"))
    }
    fn previsit_uoperation(&mut self, node: &UOperation) {
        self.enter_node(format!("UOperation::{node:?}"))
    }
    fn previsit_expr(&mut self, node: &Expr) {
        let label = match node {
            Expr::Val { .. } => "Expr::Literal".to_string(),
            Expr::BinaryOp { op, .. } => format!("Expr::Binary({op:?})"),
            Expr::UnaryOp { op, .. } => format!("Expr::Unary({op:?})"),
            Expr::Var { .. } => "Expr::Identifier".to_string(),
            Expr::CallName { .. } => "Expr::CallName".to_string(),
            Expr::CallRef { .. } => "Expr::CallRef".to_string(),
            Expr::Array { .. } => "Expr::Array".to_string(),
            Expr::Index { .. } => "Expr::Index".to_string(),
            //Expr::Neg { .. } => "Expr::Neg".to_string(),
            // Expr::Deref(..) => "Expr::Deref".to_string(),
            // Expr::Ref(..) => "Expr::Ref".to_string(),
        };
        self.enter_node(label);
    }
    fn previsit_type(&mut self, node: &Type) {
        let label = match node {
            Type::IntT => "Type::IntT",
            Type::BoolT => "Type::BoolT",
            Type::UnitT => "Type::UnitT",
            Type::ArrayT(_) => "Type::ArrayT",
        };
        self.enter_node(label);
    }

    fn previsit_stmt(&mut self, node: &Stmt) {
        let label = match node {
            Stmt::ForD { .. } => "Stmt::ForD",
            Stmt::If { .. } => "Stmt::If",
            Stmt::Assign { .. } => "Stmt::Assign",
            Stmt::ExprStmt { .. } => "Stmt::ExprStmt",
            Stmt::Decl { .. } => "Stmt::Decl",
            Stmt::Return { .. } => "Stmt::Return",
            Stmt::Block { .. } => "Stmt::Block",
            Stmt::Continue => "Stmt::Continue",
            Stmt::Break => "Stmt::Break",
            Stmt::While { .. } => "Stmt::While",
        };
        self.enter_node(label);
    }
    fn previsit_arguments(&mut self, _node: &Arguments) {
        self.enter_node("Arguments")
    }
    fn previsit_paramlist(&mut self, _node: &ParamList) {
        self.enter_node("Paramlist")
    }
    fn previsit_fun(&mut self, _node: &Fun) {
        self.enter_node("Function")
    }

    fn postvisit_name(&mut self, _node: &Name) {
        self.exit_node();
    }
    fn postvisit_value(&mut self, _node: &Value) {
        self.exit_node();
    }
    fn postvisit_operation(&mut self, _node: &Operation) {
        self.exit_node();
    }
    fn postvisit_uoperation(&mut self, _node: &UOperation) {
        self.exit_node();
    }
    fn postvisit_expr(&mut self, _node: &Expr) {
        self.exit_node();
    }
    fn postvisit_type(&mut self, _node: &Type) {
        self.exit_node();
    }
    fn postvisit_stmt(&mut self, _node: &Stmt) {
        self.exit_node();
    }
    fn postvisit_arguments(&mut self, _node: &Arguments) {
        self.exit_node();
    }
    fn postvisit_paramlist(&mut self, _node: &ParamList) {
        self.exit_node();
    }
    fn postvisit_fun(&mut self, _node: &Fun) {
        self.exit_node();
    }
    fn postvisit_program(&mut self, _node: &Program) {
        self.exit_node();
    }
}
