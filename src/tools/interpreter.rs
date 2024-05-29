use crate::visit::ExprVisitor;
use crate::ast::{Unary, Binary, Grouping, Expr, Literal, Comma, Ternary};
use crate::token_type::TokenType;

pub struct Interpreter;

impl ExprVisitor<Literal> for Interpreter {

    fn visit_expr(&mut self, e: &Expr) -> Literal {
        match e {
            Expr::UNARY(u) => self.visit_unary(&u),
            Expr::BINARY(b) => self.visit_binary(&b),
            Expr::GROUPING(g) => self.visit_grouping(&g),
            Expr::LITERAL(l) => self.visit_literal(&l),
            Expr::COMMA(c) => self.visit_comma(&c),
            Expr::TERNARY(t) => self.visit_ternary(&t),
        } 
    }

    fn visit_literal(&mut self, l: &Literal) -> Literal {
        return l;
    }

    fn visit_unary(&mut self, u: &Unary) -> Literal {
        let r = self.visit_expr(&u.rhs);

        return match (u.op.kind, r) {
            (TokenType::MINUS, Literal::NUMBER(val)) => Literal::new(-val),
            (TokenType::BANG, Literal::BOOL(val)) => Literal::new(!val),
            (TokenType::BANG, Literal::NIL) => Literal::new(false),
            (TokenType::BANG, _) => Literal::new(true),
            _ => panic!("TODO"),
        }
    }

    fn visit_binary(&mut self, b: &Binary) -> Literal {
        let r = self.visit_expr(&b.rhs);
        let l = self.visit_expr(&b.lhs);

        return match (u.op.kind, l, r) {
            _ => panic!("TODO"),
        }
    }

    fn visit_comma(&mut self, c: &Comma) -> Literal {
        unimplemented!();
    }

    fn visit_ternary(&mut self, t: &Ternary) -> T {
        unimplemented!();
    }

    fn visit_grouping(&mut self, g: &Grouping) -> Literal {
        return self.visit_expr(&g.expr);
    }

}
