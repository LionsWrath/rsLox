use crate::ast_stmt::*;

pub trait StmtVisitor<T> {
    fn visit_stmt(&mut self, s: &Stmt) -> T;
    fn visit_expression(&mut self, e: &Expression) -> T;
    fn visit_print(&mut self, p: &Print) -> T;
}
