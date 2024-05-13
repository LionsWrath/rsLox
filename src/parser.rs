use crate::ast::*;
use crate::token::{Token, ValueTypes};
use crate::token_type::TokenType;
use crate::error::ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let current: usize = 0;
    
        Parser {
            tokens,
            current,
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenType::EOF
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1; 
        }

        self.previous()
    }

    fn check(&self, tt: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().kind == tt
    }

    fn match_types(&mut self, types: Vec<TokenType>) -> bool {
        for tt in types {
            if self.check(tt) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn expression(&mut self) -> Expr {
        self.comma()
    }

    fn comma(&mut self) -> Expr {
        let mut expr: Expr = self.equality();

        while self.match_types(vec![
            TokenType::COMMA,
        ]) {
            let rhs: Expr = self.equality();
            expr = Expr::COMMA(
                Comma::new(
                    Box::new(expr),
                    Box::new(rhs),
                )
            )
        }

        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.match_types(vec![
            TokenType::BANGEQUAL,
            TokenType::EQUALEQUAL,
        ]) {
            let op: Token = self.previous();
            let rhs: Expr = self.comparison();
            expr = Expr::BINARY(
                Binary::new(
                    op,
                    Box::new(expr),
                    Box::new(rhs),
                )
            )
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while self.match_types(vec![
            TokenType::GREATER,
            TokenType::GREATEREQUAL,
            TokenType::LESS,
            TokenType::LESSEQUAL,
        ]) {
            let op: Token = self.previous();
            let rhs: Expr = self.term();
            expr = Expr::BINARY(
                Binary::new(
                    op,
                    Box::new(expr),
                    Box::new(rhs),
                )
            ) 
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();

        while self.match_types(vec![
            TokenType::MINUS,
            TokenType::PLUS,
        ]) {
            let op: Token = self.previous();
            let rhs: Expr = self.factor();
            expr = Expr::BINARY(
                Binary::new(
                    op,
                    Box::new(expr),
                    Box::new(rhs),
                )
            )
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();

        while self.match_types(vec![
            TokenType::SLASH,
            TokenType::STAR,
        ]) {
            let op: Token = self.previous();
            let rhs: Expr = self.unary();
            expr = Expr::BINARY(
                Binary::new(
                    op,
                    Box::new(expr),
                    Box::new(rhs),
                )
            )
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        while self.match_types(vec![
            TokenType::BANG,
            TokenType::MINUS,
        ]) {
            let op: Token = self.previous();
            let rhs: Expr = self.unary();
            return Expr::UNARY(
                Unary::new(
                    op,
                    Box::new(rhs),
                )
            )
        }

        match self.primary() {
            Ok(parsed) => return parsed,
            Err(err) => panic!("[PARSER] {}", err)
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {

        if self.match_types(vec![
            TokenType::NIL,
        ]) {
            return Ok(Expr::LITERAL(Literal::NIL))
        }

        if self.match_types(vec![
            TokenType::STRING,
            TokenType::NUMBER,
            TokenType::FALSE,
            TokenType::TRUE,
        ]) {
            return match self.previous().get_value() {
                Some(ValueTypes::NUMBER(value)) => Ok(Expr::LITERAL(Literal::NUMBER(value))),
                Some(ValueTypes::STRING(value)) => Ok(Expr::LITERAL(Literal::STRING(value))),
                Some(ValueTypes::BOOL(value)) => Ok(Expr::LITERAL(Literal::BOOL(value))),
                _ => Err(ParseError::new("Expect number, string or bool".to_string(), self.previous())),
            }
        }

        if self.match_types(vec![
            TokenType::LEFTPAREN,
        ]) {
            let expr = self.expression();
            match self.consume(TokenType::RIGHTPAREN, "Expect ')' after expression") {
                Ok(_) => (),
                Err(err) => panic!("[PARSER] {}", err)
            };
            return Ok(Expr::GROUPING(
                Grouping::new(
                    Box::new(expr)
                )
            ));
        }

        return Err(ParseError::new("Expect expression".to_string(), self.peek().clone()));
    }

    fn consume(&mut self, tt: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(tt) {
            return Ok(self.advance());
        }

        Err(ParseError::new(message.to_string(), self.peek().clone()))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == TokenType::SEMICOLON { return; }

            match self.peek().kind {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR 
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => self.advance(),
            };
        }

    }

}
