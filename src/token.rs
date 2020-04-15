use std::{
    collections::HashMap,
    fmt
};

#[derive(Debug)]
pub struct Token<T> {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<T>,
    pub line: i32
}

impl<T> fmt::Display for Token<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ttype:{:#?} lexeme:{} literal:{:#?}", self.ttype, self.lexeme, self.literal)
    }
}

#[derive(Debug)]
pub enum TokenType {                                   
    // Single-character tokens.                      
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR, 

    // One or two character tokens.                  
    BANG, BANG_EQUAL,                                
    EQUAL, EQUAL_EQUAL,                              
    GREATER, GREATER_EQUAL,                          
    LESS, LESS_EQUAL,                                

    // Literals.                                     
    IDENTIFIER, STRING, NUMBER,                      

    // Keywords.                                     
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,  
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,    

    EOF                                              
}
