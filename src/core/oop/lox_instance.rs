use core::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use rustc_hash::FxHashMap;
use crate::core::error_types::runtime_error::RuntimeError;
use crate::core::syntax::components::expression::LoxValue;
use crate::core::syntax::token::Token;

use super::lox_class::LoxClass;
use std::fmt::Display;
use std::fmt::Formatter;
#[derive(Clone)]
pub struct LoxInstance {
    pub lox_class: LoxClass,
    fields: FxHashMap<String, LoxValue>
}

impl LoxInstance {
    pub fn new(lox_class: LoxClass) -> LoxInstance {
        Self {
            lox_class,
            fields: FxHashMap::default()
        }
    }
    pub fn get(&self, r: Rc<RefCell<Self>>, name: &Token) -> Result<LoxValue, RuntimeError> {
        if let Some(v) = self.fields.get(&name.lexeme) {
            return Ok(v.clone());
        }
    
        let method = self.lox_class.find_method(&name.lexeme);
        if method != LoxValue::Nil {
            if let LoxValue::LoxFunction(f) = method {
                return Ok(
                    LoxValue::LoxFunction(
                        f.bind(Rc::clone(&r))?.into()
                    )
                )
            }
        }
    
        Err(RuntimeError::UndefinedProperty())
    }
    

    pub fn set(&mut self, name: Token, value: LoxValue) {
        self.fields.insert(name.lexeme, value);
    }


}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "LoxInstance({})", self.lox_class.name)
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