use std::fmt;
use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Clone, Debug)]
pub struct ParseError {
    pub message: String,
    pub token: Token,
}

impl ParseError {
    pub fn new(message: String, token: Token) -> Self {
        ParseError {
            message,
            token
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.token.kind == TokenType::EOF {
            write!(f, "{} at end {}", self.token.line, self.message)
        } else {
            write!(f, "{} at '{}' {}", self.token.line, self.token.get_lexeme(), self.message)
        }
    }
}
