use super::token::{ Token,
    TokenType,
    TokenType::*,
};

use std::convert::TryInto;

use crate::Rox;

#[derive(Default)]
pub struct Scanner<T> {
    source: String,
    chars: Vec<char>,
    tokens: Vec<Token<T>>,
    start: i32,
    current: i32,
    line: i32
}

impl<T: std::default::Default> Scanner<T> {
    pub fn new(source: &String) -> Self {
        Scanner {
            source: source.clone(),
            chars: source.clone().chars().collect(),
            line: 1,
            ..Default::default()
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token<T>> {
        while !self.is_at_end() {
          self.start = self.current;
          self.scan_token();
        }

        self.tokens.push(Token {
            ttype: EOF,
            lexeme: String::from(""),
            literal: None,
            line: self.line
        });

        &self.tokens
    }

    fn scan_token(&mut self) {
        let ch = self.advance();

        match ch {
            '(' => self.add_token(LEFT_PAREN, None),
            ')' => self.add_token(RIGHT_PAREN, None),
            '{' => self.add_token(LEFT_BRACE, None),
            '}' => self.add_token(RIGHT_BRACE, None),
            ',' => self.add_token(COMMA, None),
            '.' => self.add_token(DOT, None),
            '-' => self.add_token(MINUS, None),
            '+' => self.add_token(PLUS, None),
            ';' => self.add_token(SEMICOLON, None),
            '*' => self.add_token(STAR, None),
            ' ' => return,
            '\r' => return,
            '\t' => return,
            '\n' => self.line += 1,
            '!' => if self._match(&'=') {
                self.add_token(BANG_EQUAL, None)
            } else {
                self.add_token(BANG, None)
            },
            '=' => if self._match(&'=') {
                self.add_token(EQUAL_EQUAL, None)
            } else {
                self.add_token(EQUAL, None)
            },
            '<' => if self._match(&'=') {
                self.add_token(LESS_EQUAL, None)
            } else {
                self.add_token(LESS, None)
            },
            '>' => if self._match(&'=') {
                self.add_token(GREATER_EQUAL, None)
            } else {
                self.add_token(GREATER, None)
            },
            '/' => if self._match(&'/') {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            } else {
                self.add_token(SLASH, None)
            },
            _ => println!("No match"),
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.chars[self.current as usize - 1]
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            '\0';
        }

        self.chars[self.current as usize]
    }

    fn add_token(&mut self, ttype: TokenType, literal: Option<T>) {
        let chars = &self.chars[self.start as usize..self.current as usize];
        let text = chars.into_iter().collect();

        self.tokens.push(Token {
            ttype,
            lexeme: text,
            literal: literal,
            line: self.line
        });
    }

    fn parse_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }

            self.advance();
        }

        if self.is_at_end() {
            Rox::error(self.line, String::from("Unterminated string."));
            return;
        }

        self.advance();

        let value = self.chars[
            self.start as usize + 1 ..self.current as usize - 1
        ];
        self.add_token(STRING, Some(value));
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
