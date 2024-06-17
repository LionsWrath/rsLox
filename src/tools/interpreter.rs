/* -------------------------------------------------------------------------------------
*   
*   Interpreter for the Language Lox
*
*   Tree-walk interpreters. It goes over each node of the AST and interpret by using
*   the visitor pattern. The order of calling the visit functions matter and dictates
*   the order of execution.
*
*   Each node can fail, resulting in a EvaluationError. For a successful execution we
*   get a literal representing a value.
*
* ------------------------------------------------------------------------------------- */


use crate::visit::ExprVisitor;
use crate::ast::{Unary, Binary, Grouping, Expr, Literal, Comma, Ternary};
use crate::token_type::TokenType;
use crate::error::EvaluationError;

pub struct Interpreter;

impl Interpreter {

    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn interpret(&mut self, e: &Expr) {
        match self.visit_expr(e) {
            Ok(lit) => println!("{}", lit.to_string()),
            Err(err) => println!("{}", err),
        }
    }
}

impl ExprVisitor<Result<Literal, EvaluationError>> for Interpreter {


    fn visit_expr(&mut self, e: &Expr) -> Result<Literal, EvaluationError> {
        match e {
            Expr::UNARY(u) => self.visit_unary(&u),
            Expr::BINARY(b) => self.visit_binary(&b),
            Expr::GROUPING(g) => self.visit_grouping(&g),
            Expr::LITERAL(l) => self.visit_literal(&l),
            Expr::COMMA(c) => self.visit_comma(&c),
            Expr::TERNARY(t) => self.visit_ternary(&t),
        } 
    }

    fn visit_literal(&mut self, l: &Literal) -> Result<Literal, EvaluationError> {
        Ok(l.clone())
    }

    fn visit_unary(&mut self, u: &Unary) -> Result<Literal, EvaluationError> {
        let r = match self.visit_expr(&u.rhs) {
            Ok(lit) => lit,
            Err(e) => return Err(e)
        };

        match (u.op.kind.clone(), r) {
            (TokenType::MINUS, Literal::NUMBER(val)) => Ok(Literal::NUMBER(-val)),
            (TokenType::BANG, Literal::BOOL(val)) => Ok(Literal::BOOL(!val)),
            (TokenType::BANG, Literal::NIL) => Ok(Literal::BOOL(true)),
            (TokenType::BANG, _) => Ok(Literal::BOOL(false)),
            (_, lit) => Err(
                EvaluationError::new_unary(
                    "Invalid operation on unary operand.".to_string(),
                    lit
                )
            ),
        }
    }

    fn visit_binary(&mut self, b: &Binary) -> Result<Literal, EvaluationError> {

        let l = match self.visit_expr(&b.lhs) {
            Ok(lit) => lit,
            Err(e) => return Err(e)
        };

        let r = match self.visit_expr(&b.rhs) {
            Ok(lit) => lit,
            Err(e) => return Err(e)
        };

        match (b.op.kind.clone(), l, r) {
            (TokenType::MINUS, Literal::NUMBER(lval), Literal::NUMBER(rval)) => Ok(Literal::NUMBER(lval - rval)),
            (TokenType::PLUS, Literal::NUMBER(lval), Literal::NUMBER(rval)) => Ok(Literal::NUMBER(lval + rval)),
            (TokenType::PLUS, Literal::STRING(lval), Literal::STRING(rval)) => {
                let mut appended = lval.clone();
                appended.push_str(&rval);
                Ok(Literal::STRING(appended))
            },
            (TokenType::PLUS, Literal::STRING(lval), Literal::NUMBER(rval)) => {
                let mut appended = lval.clone();
                appended.push_str(&rval.to_string());
                Ok(Literal::STRING(appended))
            },
            (TokenType::PLUS, Literal::NUMBER(lval), Literal::STRING(rval)) => {
                let mut appended = lval.to_string();
                appended.push_str(&rval);
                Ok(Literal::STRING(appended))
            },
            (TokenType::SLASH, Literal::NUMBER(lval), Literal::NUMBER(rval)) => {
                
                if rval == 0.0 {
                    return Err(
                        EvaluationError::new_binary(
                            "Division by zero.".to_string(),
                            Literal::NUMBER(lval),
                            Literal::NUMBER(rval),
                        )
                    )
                }
                Ok(Literal::NUMBER(lval / rval))
            },
            (TokenType::STAR, Literal::NUMBER(lval), Literal::NUMBER(rval)) => Ok(Literal::NUMBER(lval * rval)),
            (TokenType::GREATER, Literal::NUMBER(lval), Literal::NUMBER(rval)) => Ok(Literal::BOOL(lval > rval)),
            (TokenType::GREATEREQUAL, Literal::NUMBER(lval), Literal::NUMBER(rval)) => Ok(Literal::BOOL(lval >= rval)),
            (TokenType::LESS, Literal::NUMBER(lval), Literal::NUMBER(rval)) => Ok(Literal::BOOL(lval < rval)),
            (TokenType::LESSEQUAL, Literal::NUMBER(lval), Literal::NUMBER(rval)) => Ok(Literal::BOOL(lval <= rval)),
            (TokenType::BANGEQUAL, Literal::NUMBER(lval), Literal::NUMBER(rval)) => Ok(Literal::BOOL(lval != rval)),
            (TokenType::BANGEQUAL, Literal::NIL, Literal::NIL) => Ok(Literal::BOOL(false)),
            (TokenType::BANGEQUAL, Literal::NUMBER(_), Literal::NIL) => Ok(Literal::BOOL(true)),
            (TokenType::BANGEQUAL, Literal::NIL, Literal::NUMBER(_)) => Ok(Literal::BOOL(true)),
            (TokenType::EQUALEQUAL, Literal::NUMBER(lval), Literal::NUMBER(rval)) => Ok(Literal::BOOL(lval == rval)),
            (TokenType::EQUALEQUAL, Literal::NIL, Literal::NIL) => Ok(Literal::BOOL(true)),
            (TokenType::EQUALEQUAL, Literal::NUMBER(_), Literal::NIL) => Ok(Literal::BOOL(false)),
            (TokenType::EQUALEQUAL, Literal::NIL, Literal::NUMBER(_)) => Ok(Literal::BOOL(false)),
            (_, lit1, lit2) => return Err(
                EvaluationError::new_binary(
                    "Invalid operation on binary operand".to_string(),
                    lit1,
                    lit2,
                )
            ),
        }
    }

    fn visit_comma(&mut self, c: &Comma) -> Result<Literal, EvaluationError> {

        let _ = match self.visit_expr(&c.lhs) {
            Ok(lit) => lit,
            Err(e) => return Err(e)
        };

        let r = match self.visit_expr(&c.rhs) {
            Ok(lit) => lit,
            Err(e) => return Err(e)
        };

        Ok(r)
    }

    fn visit_ternary(&mut self, t: &Ternary) -> Result<Literal, EvaluationError> {

        let cond = match self.visit_expr(&t.cond) {
            Ok(lit) => lit,
            Err(e) => return Err(e)
        };

        // Everything else is true by default
        match cond {
            Literal::BOOL(false) | Literal::NIL => self.visit_expr(&t.else_expr),
            _ => self.visit_expr(&t.then_expr),
        }
    }

    fn visit_grouping(&mut self, g: &Grouping) -> Result<Literal, EvaluationError> {
        self.visit_expr(&g.expr)
    }

}
