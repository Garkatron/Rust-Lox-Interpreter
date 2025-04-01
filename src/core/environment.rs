use super::error_types::runtime_error::RuntimeError;
use super::syntax::components::expression::LiteralValue;
use super::syntax::token::Token;
use super::syntax::token_type::TokenType;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, LiteralValue>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: &str, value: LiteralValue) -> Result<(), RuntimeError> {
        if self.values.contains_key(name) {
            return Err(RuntimeError::RedefinedVariable(name.to_owned()));
        }
        self.values.insert(name.to_owned(), value);
        Ok(())
    }

    pub fn get(&self, name: &Token) -> Result<LiteralValue, RuntimeError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.clone());
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
        }
        Err(RuntimeError::UndefinedVariable(name.clone()))
    }

    pub fn assign(&mut self, name: &Token, value: LiteralValue) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }
        if let Some(enclosing) = &mut self.enclosing {
            return enclosing.assign(name, value);
        }
        Err(RuntimeError::UndefinedVariable(name.clone()))
    }

    pub fn ancestor(&mut self, distance: usize) -> Option<&mut Environment> {
        let mut env = self;
        for _ in 0..distance {
            match env.enclosing {
                Some(ref mut enclosing) => env = enclosing,
                None => return None,
            }
        }
        Some(env)
    }

    pub fn get_at(&mut self, distance: usize, name: &str) -> Result<LiteralValue, RuntimeError> {
        match self.ancestor(distance) {
            Some(env) => env.values.get(name).cloned().ok_or_else(|| {
                RuntimeError::UndefinedVariable(Token {
                    lexeme: name.to_string(),
                    line: 0,
                    literal: LiteralValue::Nil,
                    t_type: TokenType::VAR,
                })
            }),
            None => Err(RuntimeError::UndefinedVariable(Token {
                lexeme: name.to_string(),
                line: 0,
                literal: LiteralValue::Nil,
                t_type: TokenType::VAR,
            })),
        }
    }
}
