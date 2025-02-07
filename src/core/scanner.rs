use std::collections::HashMap;

use crate::utils::colors::Color;

use super::{
    expression::LiteralValue,
    token::Token,
    token_type::TokenType::{self, *},
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    // Scanning control
    start: usize,
    current: usize,
    line: usize,

    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), AND);
        keywords.insert("class".to_string(), CLASS);
        keywords.insert("else".to_string(), ELSE);
        keywords.insert("false".to_string(), FALSE);
        keywords.insert("for".to_string(), FOR);
        keywords.insert("fun".to_string(), FUN);
        keywords.insert("if".to_string(), IF);
        keywords.insert("nil".to_string(), NIL);
        keywords.insert("or".to_string(), OR);
        keywords.insert("print".to_string(), PRINT);
        keywords.insert("super".to_string(), SUPER);
        keywords.insert("this".to_string(), THIS);
        keywords.insert("true".to_string(), TRUE);
        keywords.insert("var".to_string(), VAR);
        keywords.insert("while".to_string(), WHILE);

        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // beginning of next lexeme
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::from(
            EOF,
            "".to_string(),
            LiteralValue::Nil,
            self.line,
        ));

        // Return tokens
        self.tokens.clone()
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // 4.5 -> Recognizing Lexemes
    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            // Recognizing Lexemes
            // Normal lexemes
            '(' => self.add_token(LEFT_PAREN),
            ')' => self.add_token(RIGHT_PAREN),
            '{' => self.add_token(LEFT_BRACE),
            '}' => self.add_token(RIGHT_BRACE),
            ',' => self.add_token(COMMA),
            '.' => self.add_token(DOT),
            '-' => self.add_token(MINUS),
            '+' => self.add_token(PLUS),
            ';' => self.add_token(SEMICOLON),
            ':' => self.add_token(COLON),
            '?' => self.add_token(QUESTION_MARK),
            '*' => self.add_token(STAR),

            // Operators
            // Combination
            '!' => {
                let token_type = if self.char_match('=') {
                    BANG_EQUAL
                } else {
                    BANG
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.char_match('=') {
                    EQUAL_EQUAL
                } else {
                    EQUAL
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.char_match('=') {
                    LESS_EQUAL
                } else {
                    LESS
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.char_match('=') {
                    GREATER_EQUAL
                } else {
                    GREATER
                };
                self.add_token(token_type);
            }

            /*
            '!'=> self.add_token(if self.char_match('=') { BANG_EQUAL } else { BANG }),
            '='=> self.add_token(if self.char_match('=') { EQUAL_EQUAL } else { EQUAL }),
            '<'=> self.add_token(if self.char_match('=') { LESS_EQUAL } else { LESS }),
            '>'=> self.add_token(if self.char_match('=') { GREATER_EQUAL } else { GREATER }),
            */
            '/' => {
                if self.char_match('*') {
                    // Code for handle multiline comment
                    while !self.is_at_end() {
                        if self.peek() == '\n' {
                            self.line += 1;
                        } else if self.peek() == '*' && self.peek_next() == '/' {
                            self.current += 2; // Skip two characters if they form the closing of a multiline comment
                            return; // Finish
                        }
                        self.advance();
                    }
                    self.error("[SCANNER][ERROR]: Unfinished multiline comment.");
                } else if self.char_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(SLASH);
                }
            }

            // Spaces
            ' ' => {}
            '\r' => {}
            '\t' => {}

            // Newline
            '\n' => {
                self.line += 1;
            }

            // Longer Lexemes
            // Literals
            '"' => self.string(),

            // Default
            _ => {
                // Number literals
                if Self::is_digit(c) {
                    self.number();
                } else if Self::is_alpha(c) {
                    self.identifier()
                } else {
                    self.error("[SCANNER][ERROR]: Unexpected character.");
                }
            }
        }
    }

    fn identifier(&mut self) {
        while Self::is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text = self.source[self.start..self.current].to_string();
        let t_type: &TokenType = self.keywords.get(&text).unwrap_or(&IDENTIFIER);
        self.add_token(t_type.clone())
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }
        // Look for a fractional part.
        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();
            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }
        self.add_token_lit(
            NUMBER,
            LiteralValue::Number(self.source[self.start..self.current].parse().unwrap()),
        )
    }

    fn string(&mut self) {
        // While char isn't " and is not the end of source
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        // If not string closed
        if self.is_at_end() {
            self.error("[SCANNER][ERROR]: Unterminated String");
            return;
        }

        // The closing "
        self.advance();

        // Trim the surrounding quotes.
        let value: String = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_lit(STRING, LiteralValue::String(value))
    }

    fn char_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.source.chars().nth(self.current).map_or(false, |c| {
            if c == expected {
                self.current += 1;
                true
            } else {
                false
            }
        })
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn is_alpha_numeric(c: char) -> bool {
        return Self::is_alpha(c) || Self::is_digit(c);
    }

    fn is_alpha(c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn advance(&mut self) -> char {
        let current_char = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        current_char
    }

    fn error(&mut self, message: &str) {
        self.report(self.line, "".to_string(), message)
    }

    fn report(&mut self, line: usize, where_is: String, message: &str) {
        Color::ecprintln(
            &format!("{}, {} at line {}", message, where_is, line),
            Color::Red,
        );
    }

    fn add_token(&mut self, t_type: TokenType) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::from(t_type, lexeme, LiteralValue::Nil, self.line));
    }

    fn add_token_lit(&mut self, t_type: TokenType, literal: LiteralValue) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::from(t_type, lexeme, literal, self.line));
    }
}
