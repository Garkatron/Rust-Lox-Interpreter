use super::{expression::LiteralValue, runtime_error::RuntimeError, token::Token};
use std::cell::RefCell;
use std::rc::Rc;
use rustc_hash::FxHashMap; // ! Speed


#[derive(Debug, Clone)]
pub struct Environment {
    values: FxHashMap<String, LiteralValue>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(Environment {
            values: FxHashMap::default(),
            enclosing: enclosing,
        }))
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
            return enclosing.borrow_mut().get(name);
        }

        Err(RuntimeError::UndefinedVariable(name.clone()))
    }

    pub fn assign(&mut self, name: &Token, value: LiteralValue) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }

        if let Some(enclosing) = &mut self.enclosing {
            let _ = enclosing.borrow_mut().assign(name, value);
            return Ok(());
        }

        Err(RuntimeError::UndefinedVariable(name.clone()))
    }
}
