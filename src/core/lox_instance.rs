use core::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use super::lox_class::LoxClass;
use std::fmt::Display;
use std::fmt::Formatter;
pub struct LoxInstance {
    pub lox_class: Rc<RefCell<LoxClass>>
}

impl LoxInstance {
    pub fn new(lox_class: Rc<RefCell<LoxClass>>) -> LoxInstance {
        Self {
            lox_class
        }
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