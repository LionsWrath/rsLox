use crate::ast::*;
use crate::token::Token;
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

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        return self.peek().kind == TokenType::EOF;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1; 
        }
        return self.previous();
    }

    fn check(&self, tt: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().kind == tt;
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
        self.equality()
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

        return expr;
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

        return expr;
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

        return expr;
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

        return expr;
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

        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.match_types(vec![
            TokenType::FALSE,
            TokenType::TRUE,
            TokenType::NIL,
        ]) {
            return Expr::LITERAL(self.previous());
        }

        if self.match_types(vec![
            TokenType::LEFTPAREN,
        ]) {
            let expr = self.expression();
            match self.consume(TokenType::RIGHTPAREN, "Expect ')' after expression") {
                Ok(_) => (),
                Err(err) => panic!("[PARSER] {}", err)
            };
            return Expr::GROUPING(
                Grouping::new(
                    Box::new(expr)
                )
            );
        }

        return Expr::LITERAL(self.previous()); // PLACEHOLDER FOR NOW
    }

    fn consume(&mut self, tt: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(tt) {
            return Ok(self.advance());
        }

        Err(ParseError::new(message.to_string(), self.peek().clone()))
    }

    fn synchronize() {
        unimplemented!()
    }

}
