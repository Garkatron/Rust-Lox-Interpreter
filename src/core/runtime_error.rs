use std::fmt;

use super::token::Token;

#[derive(Debug, Clone)]
pub enum RuntimeError {
    BadOperator(Token, String),
    BadStatement(String),
    UndefinedVariable(Token)
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeError::BadOperator(operator, message) => {
                write!(f, "Bad operator '{}': {}", operator, message)
            }
            RuntimeError::BadStatement(message) => {
                write!(f, "Bad statement: {}", message)
            }
            RuntimeError::UndefinedVariable(token) => {
                write!(f, "Undefined variable: {} ", token.lexeme)
            }
        }
    }
}

impl std::error::Error for RuntimeError {}
