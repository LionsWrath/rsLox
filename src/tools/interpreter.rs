use crate::visit::ExprVisitor;
use crate::ast::{Unary, Binary, Grouping, Expr, Literal, Comma, Ternary};
use crate::token_type::TokenType;

pub struct Interpreter;

impl ExprVisitor<Literal> for Interpreter {

    fn visit_literal(&mut self, l: &Literal) -> Literal {
        return l;
    }

    fn visit_expr(&mut self, e: &Expr) -> Literal {
        unimplemented!();
    }

    fn visit_unary(&mut self, u: &Unary) -> Literal {
        let r = self.visit_expr(&u.rhs);

        match (u.op.kind, r) {
            (TokenType::MINUS, Literal::NUMBER(val)) => return Literal(-val),
            (TokenType::BANG, Literal::BOOL(val)) => return Literal(!val),
            _ => panic!("TODO: fix this"),
        }
    }

    fn visit_binary(&mut self, b: &Binary) -> Literal {
        unimplemented!();
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
