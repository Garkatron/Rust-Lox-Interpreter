use super::{expression::LiteralValue, runtime_error::RuntimeError, token::Token};
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn define(&mut self, name: &str, value: LiteralValue) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &Token) -> Result<&LiteralValue, RuntimeError> {
        if let Some(v) = self.values.get(&name.lexeme) {
            return Ok(v);
        } else {
            Err(RuntimeError::UndefinedVariable(name.clone()))
        }
    }
}
