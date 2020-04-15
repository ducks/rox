use crate::token::{
    Token,
    TokenType,
    TokenType::*,
};

use std::convert::TryInto;

use std::collections::HashMap;

#[derive(Default)]
pub struct Scanner {
    source: String,
    chars: Vec<char>,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32
}

impl Scanner {
    pub fn new(source: &String) -> Self {
        Scanner {
            source: source.clone(),
            chars: source.clone().chars().collect(),
            ..Default::default()
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
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

        &self.tokens
    }

    fn scan_token(&mut self) {
        let ch = self.advance();

        match ch {
            '(' => self.add_token(LEFT_PAREN),
            ')' => self.add_token(RIGHT_PAREN),
            '{' => self.add_token(LEFT_BRACE),
            '}' => self.add_token(RIGHT_BRACE),
            ',' => self.add_token(COMMA),
            '.' => self.add_token(DOT),
            '-' => self.add_token(MINUS),
            '+' => self.add_token(PLUS),
            ';' => self.add_token(SEMICOLON),
            '*' => self.add_token(STAR),
            '!' => if self._match(&'=') {
                self.add_token(BANG_EQUAL)
            } else {
                self.add_token(BANG)
            },
            '=' => if self._match(&'=') {
                self.add_token(EQUAL_EQUAL)
            } else {
                self.add_token(EQUAL)
            },
            '<' => if self._match(&'=') {
                self.add_token(LESS_EQUAL)
            } else {
                self.add_token(LESS)
            },
            '>' => if self._match(&'=') {
                self.add_token(GREATER_EQUAL)
            } else {
                self.add_token(GREATER)
            },
            _ => println!("No match"),
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.chars[self.current as usize - 1]
    }

    fn add_token(&mut self, ttype: TokenType) {
        let chars = &self.chars[self.start as usize..self.current as usize];
        let text = chars.into_iter().collect();

        self.tokens.push(Token {
            ttype: ttype,
            lexeme: text,
            literal: HashMap::new(),
            line: self.line
        });
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.chars().count().try_into().unwrap()
    }

    fn _match(&mut self, expected: &char) -> bool {
        if self.is_at_end() {
            return false
        };

        if self.chars[self.current as usize] != *expected {
            return false
        };

        self.current += 1;
        true
    }
}
