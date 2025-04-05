use super::error_types::runtime_error::RuntimeError;
use super::syntax::components::expression::LoxValue;
use super::syntax::token::Token;
use super::syntax::token_type::TokenType;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, LoxValue>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: &str, value: LoxValue) -> Result<(), RuntimeError> {
        if self.values.contains_key(name) {
            return Err(RuntimeError::RedefinedVariable(name.to_owned()));
        }
        self.values.insert(name.to_owned(), value);
        Ok(())
    }

    pub fn get(&self, name: &Token) -> Result<LoxValue, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => match &self.enclosing {
                Some(enclosing) => enclosing.borrow().get(name),
                None => Err(RuntimeError::UndefinedVariable(name.clone())),
            },
        }
    }
    
    pub fn assign(&mut self, name: &Token, value: LoxValue) -> Result<(), RuntimeError> {
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
        let mut env = Some(Rc::new(RefCell::new(self.clone()))); 
        for _ in 0..distance {
            env = match &env {
                Some(e) => e.borrow_mut().enclosing.take(),
                None => return None,
            };
        }
        env
    }
    
    pub fn get_at(&self, distance: usize, name: &str) -> Result<LoxValue, RuntimeError> {
        match self.ancestor(distance) {
            Some(env) => env.borrow().values.get(name).cloned().ok_or_else(|| {
                RuntimeError::UndefinedVariable(Token {
                    lexeme: name.to_string(),
                    line: 0,
                    literal: LoxValue::Nil,
                    t_type: TokenType::VAR,
                })
            }),
            None => Err(RuntimeError::UndefinedVariable(Token {
                lexeme: name.to_string(),
                line: 0,
                literal: LoxValue::Nil,
                t_type: TokenType::VAR,
            })),
        }
    }

    pub fn assing_at(&mut self, distance: usize, name: &str, value: LoxValue) {
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