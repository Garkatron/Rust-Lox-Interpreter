use super::error_types::runtime_error::RuntimeError;
use super::syntax::components::expression::LiteralValue;
use super::syntax::token::Token;
use super::syntax::token_type::TokenType;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, LiteralValue>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
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
            return enclosing.borrow().get(name);
        }
        Err(RuntimeError::UndefinedVariable(name.clone()))
    }

    pub fn assign(&mut self, name: &Token, value: LiteralValue) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }
        Err(RuntimeError::UndefinedVariable(name.clone()))
    }

    pub fn ancestor(&self, distance: usize) -> Option<Rc<RefCell<Environment>>> {
        let mut env = self.enclosing.clone();
        for _ in 0..distance {
            env = match &env {
                Some(e) => e.borrow().enclosing.clone(),
                None => return None,
            };
        }
        env
    }

    pub fn get_at(&self, distance: usize, name: &str) -> Result<LiteralValue, RuntimeError> {
        match self.ancestor(distance) {
            Some(env) => env.borrow().values.get(name).cloned().ok_or_else(|| {
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

    pub fn assing_at(&mut self, distance: usize, name: &str, value: LiteralValue) {
        match self.ancestor(distance) {
            Some(e) => {
                e.borrow_mut().values.insert(name.to_string(), value);
            }
            None => {
                panic!("Environment, Assing at function failed ")
            }
        } 
    }
}