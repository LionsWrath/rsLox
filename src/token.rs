use core::fmt::{Display, Formatter, Result};

use crate::token_type::TokenType;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Token {
    kind: TokenType,
    lexeme: Vec<char>,
    line: usize,
}

impl Token {

    pub fn new(kind: TokenType, lexeme: Vec<char>, line: usize) -> Self {
        Token {
            kind,
            lexeme,
            line
        }
    }

}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {:?} {}", self.kind, self.lexeme, self.line) 
    }
}
