use crate::{token::Token, token_type::TokenType};

const RADIX: u32 = 10;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {

    pub fn new(source: Vec<char>) -> Self {

        let tokens = Vec::new();
        let start = 0;
        let current = 0;
        let line = 1;

        Scanner {
            source,
            tokens,
            start,
            current,
            line,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        
        while !self.is_at_end() {
            self.start = self.current;
            self.scan();
        }

        self.tokens.push(Token::new(TokenType::EOF, self.line));

        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source[self.current + 1]
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source[self.current] != expected { return false; }

        self.current += 1;
        true
    }

    fn add_token(&mut self, kind: TokenType) {
        self.tokens.push(Token::new(kind, self.line))
    }

    fn check_keyword(&self, text_token: &Vec<char>, ident: String) -> TokenType {
        match text_token[..] {
            ['A', 'N', 'D'] => TokenType::AND,
            ['C', 'L', 'A', 'S', 'S'] => TokenType::CLASS,
            ['E', 'L', 'S', 'E'] => TokenType::ELSE,
            ['F', 'O', 'R'] => TokenType::FOR,
            ['F', 'U', 'N'] => TokenType::FUN,
            ['I', 'F'] => TokenType::IF,
            ['N', 'I', 'L'] => TokenType::NIL,
            ['O', 'R'] => TokenType::OR,
            ['P', 'R', 'I', 'N', 'T'] => TokenType::PRINT,
            ['R', 'E', 'T', 'U', 'R', 'N'] => TokenType::RETURN,
            ['S', 'U', 'P', 'E', 'R'] => TokenType::SUPER,
            ['T', 'H', 'I', 'S'] => TokenType::THIS,
            ['T', 'R', 'U', 'E'] => TokenType::TRUE,
            ['V', 'A', 'R'] => TokenType::VAR,
            ['W', 'H', 'I', 'L', 'E'] => TokenType::WHILE,
            _ => {
                TokenType::IDENTIFIER(ident)
            }
        }
    }

    fn scan(&mut self) {
       match self.advance() {
            '(' => self.add_token(TokenType::LEFTPAREN),
            ')' => self.add_token(TokenType::RIGHTPAREN),
            '{' => self.add_token(TokenType::LEFTBRACE),
            '}' => self.add_token(TokenType::RIGHTBRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BANGEQUAL)
                } else { 
                    self.add_token(TokenType::BANG)
                }
            },
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EQUALEQUAL)
                } else { 
                    self.add_token(TokenType::EQUAL)
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LESSEQUAL)
                } else { 
                    self.add_token(TokenType::LESS)
                }
            },
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GREATEREQUAL)
                } else { 
                    self.add_token(TokenType::GREATER)
                }
            },
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    self.block_comment();
                } else {
                    self.add_token(TokenType::SLASH);
                }
            },
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' => self.identifier(),
            _ => {
                panic!("[SCANNER] Error: Unexpected character");
            }
        } 
    }

    fn block_comment(&mut self) {
        while !self.is_at_end() {

            if self.peek() == '\n' {
                self.line += 1;
                self.advance();
            }

            if self.peek() == '/' && self.peek_next() == '*' {
                self.advance();
                self.advance();
                self.block_comment();
            } 
            
            if self.peek() == '*' && self.peek_next() == '/' {
                self.advance();
                self.advance();
                break;
            }

            self.advance();
        }
    }

    fn string(&mut self) {

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            panic!("[SCANNER] Error: Unterminated string");
        }

        self.advance();

        let str_token = self.source[self.start + 1..self.current - 1].iter().collect::<String>();
        self.add_token(TokenType::STRING(str_token));
    }

    fn number(&mut self) {

        while self.peek().is_digit(RADIX) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(RADIX) {
            self.advance();
            while self.peek().is_digit(RADIX) {
                self.advance();
            }
        }

        let str_token = self.source[self.start..self.current].iter().collect::<String>();
        self.add_token(TokenType::NUMBER(str_token.parse::<f64>().unwrap()));
    }

    fn identifier(&mut self) {

        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let ident = self.source[self.start..self.current].iter().collect::<String>();
        let ident_upper = ident.to_uppercase().chars().collect();

        self.add_token(self.check_keyword(&ident_upper, ident));
    }

}
