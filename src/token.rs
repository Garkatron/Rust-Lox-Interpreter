use std::fmt::Display;
use crate::token_type::TokenType;
use crate::object::Object;

#[derive(Clone)]

pub struct Token {
    pub t_type: TokenType,
    pub lexeme: String,
    pub literal: Object,
    pub line: usize
}
impl Token {
    pub fn new(t_type: TokenType, lexeme: String, literal: Object, line: usize) -> Token {
        Token{
            t_type,
            lexeme,
            literal,
            line
        }
    }
    pub fn from(t_type: TokenType, lexeme: String, literal: Object, line: usize) -> Token {
        Token{
            t_type,
            lexeme,
            literal,
            line
        }
    }
}

// Implementation of display to be used in format! or println!
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?} {} {:?}", self.t_type, self.lexeme, self.literal))
    }
}