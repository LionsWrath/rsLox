use crate::ast_stmt::{Stmt, Expression, Print, Var, Block};

pub trait StmtVisitor<T> {
    fn visit_stmt(&mut self, s: &Stmt) -> T;
    fn visit_expression(&mut self, e: &Expression) -> T;
    fn visit_print(&mut self, p: &Print) -> T;
    fn visit_var(&mut self, v: &Var) -> T;
    fn visit_block(&mut self, b: &Block) -> T;
}

