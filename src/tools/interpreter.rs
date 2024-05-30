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
        return l.clone();
    }

    fn visit_unary(&mut self, u: &Unary) -> Literal {
        let r = self.visit_expr(&u.rhs);

        return match (u.op.kind.clone(), r) {
            (TokenType::MINUS, Literal::NUMBER(val)) => Literal::NUMBER(-val),
            (TokenType::BANG, Literal::BOOL(val)) => Literal::BOOL(!val),
            (TokenType::BANG, Literal::NIL) => Literal::BOOL(false),
            (TokenType::BANG, _) => Literal::BOOL(true),
            _ => panic!("TODO"),
        }
    }

    fn visit_binary(&mut self, b: &Binary) -> Literal {

        let l = self.visit_expr(&b.lhs); // left operand first
        let r = self.visit_expr(&b.rhs);

        return match (b.op.kind.clone(), l, r) {
            (TokenType::MINUS, Literal::NUMBER(lval), Literal::NUMBER(rval)) => Literal::NUMBER(lval - rval),
            (TokenType::PLUS, Literal::NUMBER(lval), Literal::NUMBER(rval)) => Literal::NUMBER(lval + rval),
            (TokenType::PLUS, Literal::STRING(lval), Literal::STRING(rval)) => {
                let mut appended = lval.clone();
                appended.push_str(&rval);
                Literal::STRING(appended)
            },
            (TokenType::SLASH, Literal::NUMBER(lval), Literal::NUMBER(rval)) => Literal::NUMBER(lval / rval),
            (TokenType::STAR, Literal::NUMBER(lval), Literal::NUMBER(rval)) => Literal::NUMBER(lval * rval),
            _ => panic!("TODO"),
        }
    }

    fn visit_comma(&mut self, c: &Comma) -> Literal {
        unimplemented!();
    }

    fn visit_ternary(&mut self, t: &Ternary) -> Literal {
        unimplemented!();
    }

    fn visit_grouping(&mut self, g: &Grouping) -> Literal {
        return self.visit_expr(&g.expr);
    }

}
