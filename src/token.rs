use core::fmt::{Display, Formatter, Result};

use crate::token_type::TokenType;

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub kind: TokenType,
    pub line: usize,
    pub value: Option<String>,
}

pub enum ValueTypes {
    STRING(String),
    NUMBER(f64),
    BOOL(bool),
}

impl Token {

    pub fn new(kind: TokenType, line: usize, value: Option<String>) -> Self {
        Token {
            kind,
            line,
            value,
        }
    }

    pub fn get_lexeme(&self) -> String {
        match self.kind.clone() {
            TokenType::EOF => "".to_string(),
            kind => kind.to_string()
        } 
    }

    pub fn get_value(&self) -> Option<ValueTypes> {
        match self.kind {
            TokenType::FALSE => Some(ValueTypes::BOOL(false)),
            TokenType::TRUE => Some(ValueTypes::BOOL(true)),
            TokenType::STRING => Some(ValueTypes::STRING(self.value.clone().unwrap())),
            TokenType::IDENTIFIER => Some(ValueTypes::STRING(self.value.clone().unwrap())),
            TokenType::NUMBER => Some(
                ValueTypes::NUMBER(self.value.clone().unwrap().parse::<f64>().unwrap())
            ),
            _ => None,
        }
    }

}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {:?} {}", self.kind, self.get_lexeme(), self.line) 
    }
}
