use core::fmt::{Display, Formatter, Result};

use crate::token_type::TokenType;

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    kind: TokenType,
    line: usize,
}

impl Token {

    pub fn new(kind: TokenType, line: usize) -> Self {
        Token {
            kind,
            line
        }
    }

    pub fn get_lexeme(&self) -> String {
        match self.kind.clone() {
            TokenType::EOF => "".to_string(),
            TokenType::STRING(lexeme) => lexeme,
            TokenType::NUMBER(num) => num.to_string(),
            kind => kind.to_string()
        } 
    }

}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {:?} {}", self.kind, self.get_lexeme(), self.line) 
    }
}
