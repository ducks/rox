use std::{
    collections::HashMap,
    fmt
};

pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: HashMap<String, String>,
    line: i32
}

impl fmt::Display for Token {
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
