use std::fmt;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::ast::Literal;

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

#[derive(Clone, Debug)]
pub struct EvaluationError {
    pub message: String,
    pub literal: Literal,
}

impl EvaluationError {
    pub fn new(message: String, literal: Literal) -> Self {
        EvaluationError {
            message,
            literal
        }
    }
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.literal {
            Literal::BOOL(val) => write!(f, "{} literal: {}", val, self.message),
            Literal::NUMBER(val) => write!(f, "{} literal: {}", val, self.message),
            Literal::STRING(val) => write!(f, "{} literal: {}", val, self.message),
            Literal::NIL => write!(f, "NIL literal: {}", self.message),
        }
    }
}
