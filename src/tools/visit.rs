use crate::ast::*;
use crate::token::Token;

pub trait ExprVisitor<T> {
    fn visit_expr(&mut self, e: &Expr) -> T;
    fn visit_literal(&mut self, t: &Literal) -> T;
    fn visit_unary(&mut self, u: &Unary) -> T;
    fn visit_binary(&mut self, b: &Binary) -> T;
    fn visit_grouping(&mut self, g: &Grouping) -> T;
}
