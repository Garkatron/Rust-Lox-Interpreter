use crate::{token::Token, token_type::TokenType};

pub struct ErrorReporter {
    pub had_error: bool
}

impl ErrorReporter {

    pub fn new() -> Self {
        Self {
           had_error: false
        }
    }
    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line," ".to_string(), message)
    }

    fn report(&mut self, line: usize, where_is: String, message: &str){
        println!("Error {} at line {}\nMessage: {} ", where_is, line, message);
        self.had_error = true;
    }

    pub fn token_error(&mut self, token: &Token, message: &str) {
        if token.t_type == TokenType::EOF {
            self.report(token.line, " at end".to_owned(), message);
        } else {
            self.report(token.line, format!(" at ' {} '", token.lexeme) , message);
        }
    }

    pub fn reset(&mut self) {
        self.had_error = false
    }
}
