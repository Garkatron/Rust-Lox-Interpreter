// This file is generated automatically

use crate::token::Token;
use crate::object::Object;

pub trait Expr {
}

pub struct Binary {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

impl Binary {
    pub fn new(left: Expr , operator: Token, right: Expr ) -> Self {
        Self {
            left: left,
            operator: operator,
            right: right,
        }
    }
}
pub struct Grouping {
    pub expression: Expr,
}

impl Grouping {
    pub fn new(expression: Expr) -> Self {
        Self {expression: expression}
    }
}
pub struct Literal {
    pub value: Object,
}

impl Literal {
    pub fn new(value: Object) -> Self {
        Self {value: value}
    }
}
pub struct Unary {
    pub operator: Token,
    pub right: Expr,
}

impl Unary {
    pub fn new(operator: Token , right: Expr) -> Self {
        Self { 
            operator: operator,
            right: right
        }
    }
}
