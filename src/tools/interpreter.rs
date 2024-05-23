use crate::visit::ExprVisitor;
use crate::ast::{Unary, Binary, Grouping, Expr, Literal, Comma, Ternary};

pub struct Interpreter;

impl ExprVisitor<Literal> for Interpreter {

    fn visit_literal(&mut self, l: &Literal) -> Literal {
        return l;
    }

    fn visit_expr(&mut self, e: &Expr) -> Literal {
        unimplemented!();
    }

}
