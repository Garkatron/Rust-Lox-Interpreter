use std::error::Error;
use crate::lox::Lox;
use crate::object::Object;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::token_type::TokenType::*;

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
    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            // Normal Tokens

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

            // Combination

             '!'=> self.add_token(if self.char_match('=') { BANG_EQUAL } else { BANG }),
             '='=> self.add_token(if self.char_match('=') { EQUAL_EQUAL } else { EQUAL }),
             '<'=> self.add_token(if self.char_match('=') { LESS_EQUAL } else { LESS }),
             '>'=> self.add_token(if self.char_match('=') { GREATER_EQUAL } else { GREATER }),

            '/' => {
                if self.char_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance()
                    }
                } else {
                    self.add_token(SLASH)
                }
            }

            // Spaces
             ' ' => {}
            '\r' => {}
             '\t' => {}

            // Newline
             '\n' => { self.line += 1}

            // Literals
            '"' => {
                self.string()
            }

            // Default

            _ => {}
        }
    }
    unsafe fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            if self.is_at_end() {
                Lox::error(self.line, "Unterminated String".to_string())
            }
        }
    }
    fn char_match(&mut self, expected: char) -> bool {
        if self.is_at_end() ? {return false}
        let c = self.source.chars().nth(self.current).expect("Error on char_match");
        if c != expected ? { return false }
        self.current += 1;
        true
    }
    fn peek(&self) -> char {
        if self.is_at_end() {return '\0'}
        self.source.chars().nth(self.current).expect("Error on peek")
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