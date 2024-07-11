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
*   Some Considerations:
*       - Bools: Only False and NIL should evaluate to false, all other combinations are true;
*       - Sum on String: Concatenates the values, convert number to string;
*   
*
* ------------------------------------------------------------------------------------- */


use crate::environment::Environment;
use crate::visit_expr::ExprVisitor;
use crate::visit_stmt::StmtVisitor;
use crate::ast_expr::{Unary, Binary, Grouping, Expr, Literal, Comma, Ternary, Variable};
use crate::ast_stmt::{Expression, Stmt, Print, Var};
use crate::token_type::TokenType;
use crate::error::EvaluationError;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {

    pub fn new() -> Self {
        
        let environment = Environment::new();

        Interpreter {
            environment
        }
    }

    pub fn interpret(&mut self, s: &Stmt) -> Result<Literal, EvaluationError> {
        return self.visit_stmt(s);
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
            Expr::VARIABLE(v) => self.visit_variable(&v),
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
                            "Division by zero".to_string(),
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

        match cond {
            Literal::BOOL(false) | Literal::NIL => self.visit_expr(&t.else_expr),
            _ => self.visit_expr(&t.then_expr),
        }
    }

    fn visit_grouping(&mut self, g: &Grouping) -> Result<Literal, EvaluationError> {
        self.visit_expr(&g.expr)
    }

    fn visit_variable(&mut self, v: &Variable) -> Result<Literal, EvaluationError> {
        unimplemented!() 
    }
}

impl StmtVisitor<Result<Literal, EvaluationError>> for Interpreter  {
    fn visit_stmt(&mut self, s: &Stmt) -> Result<Literal, EvaluationError> {
        return match s {
            Stmt::EXPRESSION(e) => self.visit_expression(&e),
            Stmt::PRINT(p) => self.visit_print(&p),
            Stmt::VAR(v) => self.visit_var(&v)
        }        
    }

    fn visit_expression(&mut self, e: &Expression) -> Result<Literal, EvaluationError> {
        return self.visit_expr(&e.expr);
    } 

    fn visit_print(&mut self, p: &Print) -> Result<Literal, EvaluationError> {
        match self.visit_expr(&p.expr) {
            Ok(lit) => {
                println!("{}", lit.to_string());
                return Ok(Literal::NIL);
            },
            Err(e) => return Err(e),
        }
    }

    // This unwrap is still no good enough, improve later
    fn visit_var(&mut self, v: &Var) -> Result<Literal, EvaluationError> {

        match v.initializer {
                Some(expr) => {
                    self.environment.define(
                        v.name.value.unwrap(),
                        match self.visit_expr(&expr) {
                            Ok(value) => value,
                            Err(err) => panic!(""),
                        }
                    )
                },
                None => self.environment.define(v.name.value.unwrap(), Literal::NIL),
        }

        return Ok(Literal::NIL);
    }
}
