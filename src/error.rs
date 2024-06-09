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
pub struct UnaryEvaluationError {
    pub message: String,
    pub literal: Literal,
}

impl UnaryEvaluationError {
    pub fn new(message: String, literal: Literal) -> Self {
        UnaryEvaluationError {
            message,
            literal
        }
    }
}

impl fmt::Display for UnaryEvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.literal {
            Literal::NIL => write!(f, "NIL literal: {}", self.message),
            Literal::_(val) => write!(f, "{} literal: {}", val, self.message),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BinaryEvaluationError {
    pub message: String,
    pub literal_left: Literal,
    pub literal_right: Literal,

}

impl BinaryEvaluationError {
    pub fn new(message: String, literal_left: Literal, literal_right: Literal) -> Self {
        BinaryEvaluationError {
            message,
            literal_left,
            literal_right
        }
    }
}

impl fmt::Display for BinaryEvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.literal_left, &self.literal_right) {
            (Literal::NIL, Literal::NIL) => write!(f, "NIL literal: {}", self.message),
            (Literal::_(val_left), Literal::_(val_right)) => write!(f, "{} left literal {} right literal {}", val_left, val_right, self.message),
        }
    }
}
