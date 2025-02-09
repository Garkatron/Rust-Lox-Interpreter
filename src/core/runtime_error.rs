use std::fmt;

use super::token::Token;

#[derive(Debug, Clone)]
pub enum RuntimeError {
    BadOperator(Token, String),
    BadStatement(String),
    UndefinedVariable(Token),
    RedefinedVariable(String),
    BadExpr(),
    Break(),
    BadCallable(),
    ToMantyArguments(Token, usize, usize)
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeError::BadOperator(operator, message) => {
                write!(f, "[RUNTIME]: Bad operator '{}': {}", operator, message)
            }
            RuntimeError::BadStatement(message) => {
                write!(f, "[RUNTIME]: Bad statement: {}", message)
            }
            RuntimeError::UndefinedVariable(token) => {
                write!(f, "[RUNTIME]: Undefined variable: {} ", token.lexeme)
            }
            RuntimeError::RedefinedVariable(name) => {
                write!(f, "[RUNTIME]: Variable '{}' is already defined and cannot be redefined.", name)
            }
            RuntimeError::BadExpr() => {
                write!(f, "[RUNTIME]: Bad expr.")
            }
            RuntimeError::Break() => {
                write!(f, "[RUNTIME]: 'Break' used outside of a loop/while/for.")
            }
            RuntimeError::BadCallable() => {
                write!(f, "[RUNTIME]: Can only call functions and classes.")
            }
            RuntimeError::ToMantyArguments(paren, arity , args_size) => {
                write!(f, "[RUNTIME]: {} Expected {} arguments but got {}.", paren, arity, args_size)
        
            }
        }
    }
}

impl std::error::Error for RuntimeError {}
