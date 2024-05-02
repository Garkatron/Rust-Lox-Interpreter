use std::error::Error;
use crate::object::Object;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::token_type::TokenType::EOF;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}

impl Scanner {

    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while (!self.is_at_end()) {
            // beginning of next lexeme
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::from(EOF, "".to_string(), Object::None(None), self.line));
        self.tokens.iter().cloned().collect()

    }
    fn is_at_end(&self) -> bool{
        self.current >= self.source.len()
    }

    // 4.5 -> Recognizing Lexemes
    fn scan_token(&self) {
        let c: char = self.advance();
        match c {
            '(' => todo!(),
            _ => {}
        }

    }
    fn advance(&self) -> char {
        self.source.chars().nth(self.current).expect("Error on advance")
    }
    fn add_token(&mut self, t_type: TokenType) {
        self.tokens.push(Token::from(t_type,"".to_string(),Object::None(None), 0))
    }
    fn add_token_lit(&mut self, t_type: TokenType, literal: Object) {
        let text: &str = &self.source[self.start..self.current];
        self.tokens.push(Token::from(t_type,text.to_string(),literal, self.line))
    }


}