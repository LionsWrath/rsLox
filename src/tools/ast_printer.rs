use crate::visit_expr::ExprVisitor;
use crate::ast_expr::{Unary, Binary, Grouping, Expr, Literal, Comma, Ternary};
use crate::visit_stmt::StmtVisitor;
use crate::ast_stmt::{Stmt, Expression, Print};

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {}
    }

    pub fn printer(&mut self, stmt: &Stmt) -> String {
        return self.visit_stmt(stmt);
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_expr(&mut self, e: &Expr) -> String {
        match e {
            Expr::UNARY(u) => self.visit_unary(&u),
            Expr::BINARY(b) => self.visit_binary(&b),
            Expr::GROUPING(g) => self.visit_grouping(&g),
            Expr::LITERAL(l) => self.visit_literal(&l),
            Expr::COMMA(c) => self.visit_comma(&c),
            Expr::TERNARY(t) => self.visit_ternary(&t),
        } 
    }

    fn visit_comma(&mut self, c: &Comma) -> String {
        return format!(
            "({} {})",
            self.visit_expr(&c.lhs),
            self.visit_expr(&c.rhs)
        )
    }

    fn visit_ternary(&mut self, t: &Ternary) -> String {
        return format!(
            "(TERNARY IF {} THEN {} ELSE {})",
            self.visit_expr(&t.cond),
            self.visit_expr(&t.then_expr),
            self.visit_expr(&t.else_expr),
        ) 
    }

    fn visit_unary(&mut self, u: &Unary) -> String {
        return format!("({} {})", u.op.get_lexeme(), self.visit_expr(&u.rhs));
    }

    fn visit_binary(&mut self, b: &Binary) -> String {
        return format!(
            "({} {} {})",
            b.op.get_lexeme(),
            self.visit_expr(&b.lhs),
            self.visit_expr(&b.rhs)
        )
    }
    
    fn visit_grouping(&mut self, g: &Grouping) -> String {
        return format!("(GROUP {})", self.visit_expr(&g.expr));
    }

    fn visit_literal(&mut self, t: &Literal) -> String { 
        return match t {
            Literal::BOOL(value) => value.to_string(),
            Literal::NUMBER(value) => value.to_string(),
            Literal::STRING(value) => value.clone(),
            Literal::NIL => "nil".to_string(),
        };
    }
}

impl StmtVisitor<String> for AstPrinter {
    fn visit_stmt(&mut self, s: &Stmt) -> String {
        return match s {
            Stmt::PRINT(e) => self.visit_print(&e),
            Stmt::EXPRESSION(p) => self.visit_expression(&p),
        }
    }

    fn visit_expression(&mut self, e: &Expression) -> String {
        return format!(
            "(Expression {})",
            self.visit_expr(&e.expr)
        )
    }

    fn visit_print(&mut self, p: &Print) -> String {
        return format!(
            "(Print {})",
            self.visit_expr(&p.expr) 
        )
    }
}
