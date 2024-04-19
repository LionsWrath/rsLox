use std::fmt::{Display, Formatter, Result};

#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    BANG,
    BANGEQUAL,
    EQUAL,
    EQUALEQUAL,
    GREATER,
    GREATEREQUAL,
    LESS,
    LESSEQUAL,
    IDENTIFIER(String),
    STRING(String),
    NUMBER(f64),
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TokenType::STRING(_) => write!(f, "STRING"),
            TokenType::IDENTIFIER(_) => write!(f, "IDENTIFIER"),
            TokenType::NUMBER(_) => write!(f, "NUMBER"),
            _ => write!(f, "{:?}", self),
        }
    }
}
