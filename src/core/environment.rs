use super::{expression::LiteralValue, runtime_error::RuntimeError, token::Token};
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new()
        }
    }

    pub fn define(&mut self, name: &str, value: LiteralValue) -> Result<(), RuntimeError>{
        if self.values.contains_key(name) {
            return Err(RuntimeError::RedefinedVariable(name.to_string()))
        }
        self.values.insert(name.to_string(), value);
        Ok(())
    }

    pub fn get(&self, name: &Token) -> Result<LiteralValue, RuntimeError> {
        if let Some(v) = self.values.get(&name.lexeme) {
            return Ok(v.clone());
        } else {
            Err(RuntimeError::UndefinedVariable(name.clone()))
        }
    }
}