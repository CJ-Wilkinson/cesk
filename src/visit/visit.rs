use crate::ast::*;

pub trait Traverse {
    fn traverse<V: Visitor>(&self, v: &mut V);
}

pub trait Visitor {
    /*
    Name
    Value
    Operation
    Expr
    Type
    Stmt
    Arguments
    ParamList
    Fun
    Program


    name
    value
    operation
    expr
    type
    stmt
    arguments
    paramlist
    fun
    program
     */

    fn previsit_name(&mut self, _node: &Name) {}
    fn previsit_value(&mut self, _node: &Value) {}
    fn previsit_operation(&mut self, _node: &Operation) {}
    fn previsit_uoperation(&mut self, _node: &UOperation) {}
    fn previsit_expr(&mut self, _node: &Expr) {}
    fn previsit_type(&mut self, _node: &Type) {}
    fn previsit_stmt(&mut self, _node: &Stmt) {}
    fn previsit_arguments(&mut self, _node: &Arguments) {}
    fn previsit_paramlist(&mut self, _node: &ParamList) {}
    fn previsit_fun(&mut self, _node: &Fun) {}
    fn previsit_program(&mut self, _node: &Program) {}

    fn postvisit_name(&mut self, _node: &Name) {}
    fn postvisit_value(&mut self, _node: &Value) {}
    fn postvisit_operation(&mut self, _node: &Operation) {}
    fn postvisit_uoperation(&mut self, _node: &UOperation) {}
    fn postvisit_expr(&mut self, _node: &Expr) {}
    fn postvisit_type(&mut self, _node: &Type) {}
    fn postvisit_stmt(&mut self, _node: &Stmt) {}
    fn postvisit_arguments(&mut self, _node: &Arguments) {}
    fn postvisit_paramlist(&mut self, _node: &ParamList) {}
    fn postvisit_fun(&mut self, _node: &Fun) {}
    fn postvisit_program(&mut self, _node: &Program) {}
}

impl Traverse for Name {
    fn traverse<V: Visitor>(&self, v: &mut V) {
        v.previsit_name(self);
        v.postvisit_name(self);
    }
}

impl Traverse for Value {
    fn traverse<V: Visitor>(&self, v: &mut V) {
        v.previsit_value(self);
        v.postvisit_value(self);
    }
}

impl Traverse for Operation {
    fn traverse<V: Visitor>(&self, v: &mut V) {
        v.previsit_operation(self);
        v.postvisit_operation(self);
    }
}

impl Traverse for UOperation {
    fn traverse<V: Visitor>(&self, v: &mut V) {
        v.previsit_uoperation(self);
        v.postvisit_uoperation(self);
    }
}

impl Traverse for Expr {
    fn traverse<V: Visitor>(&self, v: &mut V) {
        use Expr::*;
        v.previsit_expr(self);
        match self {
            Val { value } => value.traverse(v),
            //Neg(expr) => expr.traverse(v),
            BinaryOp { lhs, op, rhs } => {
                lhs.traverse(v);
                op.traverse(v);
                rhs.traverse(v);
            }
            UnaryOp { op, expr } => {
                op.traverse(v);
                expr.traverse(v);
            }
            Var { name } => name.traverse(v),
            CallName { callee, args } => {
                callee.traverse(v);
                for arg in args.slice_ref() {
                    arg.traverse(v);
                }
            }
            CallRef { fun: _, args } => {
                for arg in args.slice_ref() {
                    arg.traverse(v);
                }
            }
            Array { elements } => {
                for elem in elements {
                    elem.traverse(v)
                }
            }
            Index { array: _, index } => {
                //name.traverse(v);
                index.traverse(v);
            } // Deref(expr) => expr.traverse(v),
              // Ref(name) => name.traverse(v),
        }
        v.postvisit_expr(self);
    }
}

impl Traverse for Type {
    fn traverse<V: Visitor>(&self, v: &mut V) {
        use Type::*;
        v.previsit_type(self);

        match self {
            IntT => {}
            BoolT => {}
            UnitT => {}
            ArrayT(type_) => type_.traverse(v),
        }
        v.postvisit_type(self);
    }
}
impl Traverse for Stmt {
    fn traverse<V: Visitor>(&self, v: &mut V) {
        use Stmt::*;
        v.previsit_stmt(self);

        match self {
            //DeclD(type_, name, expr) => {
            //type_.traverse(v);
            //name.traverse(v);
            //if let Some(expr) = expr {
            //expr.traverse(v);
            //}
            //}
            ForD {
                init,
                condition,
                update,
                body,
            } => {
                if let Some(init) = init {
                    init.traverse(v);
                }
                condition.traverse(v);

                body.traverse(v);
                if let Some(update) = update {
                    update.traverse(v);
                }
            }
            While { condition, body } => {
                condition.traverse(v);
                body.traverse(v);
            }

            If {
                condition,
                then_branch: tb,
                else_branch: fb,
            } => {
                condition.traverse(v);
                tb.traverse(v);
                if let Some(fb) = fb {
                    fb.traverse(v);
                }
            }

            Assign {
                lhs: val,
                rhs: expr,
            } => {
                val.traverse(v);
                expr.traverse(v);
            }

            ExprStmt { expr } => expr.traverse(v),
            Decl { name } => name.traverse(v),
            Return { expr } => expr.traverse(v),
            Block { stmts: v_stmts } => {
                for stmt in v_stmts.iter() {
                    stmt.traverse(v);
                }
            }
            Continue => {}
            Break => {}
            _ => todo!(),
        }
        v.postvisit_stmt(self);
    }
}

impl Traverse for Arguments {
    fn traverse<V: Visitor>(&self, v: &mut V) {
        v.previsit_arguments(self);
        for arg in self.args.iter() {
            arg.traverse(v);
        }

        v.postvisit_arguments(self);
    }
}
impl Traverse for ParamList {
    fn traverse<V: Visitor>(&self, v: &mut V) {
        v.previsit_paramlist(self);
        for (type_, name) in self.params.iter() {
            type_.traverse(v);
            name.traverse(v);
        }

        v.postvisit_paramlist(self);
    }
}

impl Traverse for Fun {
    fn traverse<V: Visitor>(&self, v: &mut V) {
        v.previsit_fun(self);

        self.typ.traverse(v);
        self.name.traverse(v);
        self.params.traverse(v);
        self.body.traverse(v);

        v.postvisit_fun(self);
    }
}
impl Traverse for Program {
    fn traverse<V: Visitor>(&self, v: &mut V) {
        v.previsit_program(self);

        for fun in self.funs.values() {
            fun.traverse(v);
        }

        v.postvisit_program(self);
    }
}
