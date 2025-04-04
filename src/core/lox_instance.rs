use core::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use rustc_hash::FxHashMap;

use super::error_types::runtime_error::RuntimeError;
use super::lox_class::LoxClass;
use super::syntax::components::expression::LiteralValue;
use super::syntax::token::Token;
use std::fmt::Display;
use std::fmt::Formatter;
pub struct LoxInstance {
    pub lox_class: Rc<RefCell<LoxClass>>,
    fields: FxHashMap<String, LiteralValue>
}

impl LoxInstance {
    pub fn new(lox_class: Rc<RefCell<LoxClass>>) -> LoxInstance {
        Self {
            lox_class,
            fields: FxHashMap::default()
        }
    }

    pub fn get(&self, name: &Token) -> Result<LiteralValue, RuntimeError> {
        if let Some(v) = self.fields.get(&name.lexeme) {
            Ok(v.clone())
        } else {
            Err(RuntimeError::UndefinedProperty())
        }
    }

    pub fn set(&mut self, name: Token, value: LiteralValue) {
        self.fields.insert(name.lexeme, value);
    }
    
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "LoxInstance with class: {}", self.lox_class.borrow().name)
    }
}

/*
use super::lox_class::LoxClass;

pub struct LoxInstance {
    lox_class: &'a LoxClass
}

impl LoxInstance {
    pub fn new(lox_class: &'a LoxClass) -> LoxInstance {
        Self {
            lox_class
        }
    }
}
*/