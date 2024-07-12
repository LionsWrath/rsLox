use std::fmt;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::ast_expr::Literal;

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

#[derive(Clone, PartialEq, Debug)]
pub enum Operands {
    UNARY(Literal),
    BINARY(Literal, Literal),
    VAR,
}

#[derive(Clone, Debug)]
pub struct EvaluationError {
    pub message: String,
    pub operands: Operands,
}

impl EvaluationError {
    fn new(message: String, operands: Operands) -> Self {
        EvaluationError {
            message,
            operands,
        }
    }

    pub fn new_unary(message: String, lit: Literal) -> Self {
        EvaluationError::new(message, Operands::UNARY(lit))
    }

    pub fn new_binary(message: String, lit1: Literal, lit2: Literal) -> Self {
        EvaluationError::new(message, Operands::BINARY(lit1, lit2))
    }

    pub fn new_var(message: String) -> Self {
        EvaluationError::new(message, Operands::VAR)
    }
    
    fn literal_to_message(lit: Literal) -> String {
        match &lit {
            Literal::NIL => "NIL".to_string(),
            Literal::BOOL(val) => val.to_string(),
            Literal::NUMBER(val) => val.to_string(),
            Literal::STRING(val) => val.to_string()
        }
    }
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.operands {
            Operands::UNARY(lit) => write!(
                f,
                "{} literal - {}",
                EvaluationError::literal_to_message(lit.clone()),
                self.message
            ),
            Operands::BINARY(lit1, lit2) => write!(
                f,
                "{} {} literal - {}",
                EvaluationError::literal_to_message(lit1.clone()),
                EvaluationError::literal_to_message(lit2.clone()),
                self.message
            ),
            Operands::VAR => write!(
                f,
                "VAR - {}",
                self.message
            ) 
        }
    }
}


#[derive(Clone, Debug)]
pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new(message: String) -> Self {
        RuntimeError {
            message,
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.message
        )
    }
}
