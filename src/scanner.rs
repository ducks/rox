use crate::token::{ 
    Token, 
    TokenType,
    TokenType::*,
};

use std::convert::TryInto;

use std::collections::HashMap;

pub struct Scanner {
    source: String,
    chars: Vec<char>,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32
}

impl Default for Scanner {
    fn default() -> Self { 
        Scanner { 
            source: String::from(""),
            chars: Vec::new(),
            tokens: Vec::new(),
            start: 0, 
            current: 0, 
            line: 1,
        } 
    }
}

impl Scanner {
    pub fn new(&self) -> Self {
        Scanner {
            source: self.source,
            chars: self.source.chars().collect(),
            ..Default::default()
        }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        while !self.is_at_end() {
          self.start = self.current;
          self.scan_token();
        }

        self.tokens.push(Token { 
            ttype: EOF, 
            lexeme: String::from(""),
            literal: HashMap::new(),
            line: self.line
        });

        self.tokens
    }

    fn scan_token(&self) {
        let ch = self.advance();

        match ch {
            '(' => self.add_token(LEFT_PAREN),
            ')' => self.add_token(RIGHT_PAREN),
            _ => println!("No match"),
        }
    }

    fn advance(&self) {
        self.current += 1;
        self.chars[self.current]
    }

    fn add_token(&self, ttype: TokenType) {
        let text = self.chars[self.start..self.current];

        self.tokens.push(Token { 
            ttype: ttype,
            lexeme: text,
            literal: HashMap::new(),
            line: self.line
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count().try_into().unwrap()
    }
}
