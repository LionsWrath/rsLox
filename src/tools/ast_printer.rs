use crate::visit::ExprVisitor;
use crate::ast::{Unary, Binary, Grouping, Expr};
use crate::token::Token;

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {}
    }

    pub fn printer(&mut self, expr: &Expr) -> String {
        return self.visit_expr(expr);
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_expr(&mut self, e: &Expr) -> String {
        match e {
            Expr::UNARY(u) => self.visit_unary(&u),
            Expr::BINARY(b) => self.visit_binary(&b),
            Expr::GROUPING(g) => self.visit_grouping(&g),
            Expr::LITERAL(l) => self.visit_literal(&l),
        } 
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
        return format!("({})", self.visit_expr(&g.expr));
    }

    fn visit_literal(&mut self, t: &Token) -> String {
       return t.to_string();
    }
}
