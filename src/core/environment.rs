use super::{expression::LiteralValue, runtime_error::RuntimeError, token::Token};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, LiteralValue>,
    enclosing: Option<Box<Environment>>
}

impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: enclosing.map(Box::new),
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
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.clone());
        }
    
        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
        }
    
        Err(RuntimeError::UndefinedVariable(name.clone()))
    }
    


    pub fn assing(&mut self, name: &Token, value: LiteralValue) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(())
        }

        if let Some(enclosing) = &mut self.enclosing {
            let _ = enclosing.assing(name, value);
            return Ok(());
        }
        
        Err(RuntimeError::UndefinedVariable(name.clone()))
    }
}