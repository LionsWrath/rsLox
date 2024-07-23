use crate::visit_expr::ExprVisitor;
use crate::ast_expr::{Unary, Binary, Grouping, Expr, Literal, Comma, Ternary, Variable};
use crate::visit_stmt::StmtVisitor;
use crate::ast_stmt::{Stmt, Expression, Print, Var};
use crate::error::EvaluationError;

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
            Expr::ASSIGN(a) => self.visit_assign(&a),
            Expr::GROUPING(g) => self.visit_grouping(&g),
            Expr::LITERAL(l) => self.visit_literal(&l),
            Expr::COMMA(c) => self.visit_comma(&c),
            Expr::TERNARY(t) => self.visit_ternary(&t),
            Expr::VARIABLE(v) => self.visit_variable(&v),
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

    fn visit_variable(&mut self, v: &Variable) -> String {
        
        let name = match v.name.value {
            Some(ref n) => n.clone(),
            None => panic!("No name in token defined for variable"),
        };

        format!("(VARIABLE {})", name)
    }

    fn visit_assign(&mut self, a: &crate::ast_expr::Assign) -> String {
        unimplemented!()
    }
}

impl StmtVisitor<String> for AstPrinter {
    fn visit_stmt(&mut self, s: &Stmt) -> String {
        return match s {
            Stmt::PRINT(e) => self.visit_print(&e),
            Stmt::EXPRESSION(p) => self.visit_expression(&p),
            Stmt::VAR(v) => self.visit_var(&v),
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

    fn visit_var(&mut self, v: &Var) -> String {
        
        let name = match v.name.value {
            Some(ref n) => n.clone(),
            None => panic!("No name in token defined for variable"),
        };

        match &v.initializer {
            Some(expr) => format!("(VAR {} {})", name, self.visit_expr(&expr)),
            None => format!("(VAR {} NIL)", name)
        }

    }
}
