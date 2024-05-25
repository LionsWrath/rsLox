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

    fn visit_unary(&mut self, u: &Unary) -> Literal {
        unimplemented!();
    }

    fn visit_binary(&mut self, b: &Binary) -> Literal {
        unimplemented!();
    }

    fn visit_comma(&mut self, c: &Comma) -> Literal {
        unimplemented!();
    }

    fn visit_grouping(&mut self, g: &Grouping) -> Literal {
        return self.visit_expr(&g.expr);
    }

}
