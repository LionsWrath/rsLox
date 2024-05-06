use crate::ast::*;
use crate::token::Token;
use crate::token_type::TokenType;

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

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        return self.peek().kind == TokenType::EOF;
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1; 
        }
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
                op,
                Box::new(expr),
                Box::new(rhs),
            ) 
        }

        return expr;
    }

    fn term(&mut self) -> Expr {
        unimplemented!();
    }

}
