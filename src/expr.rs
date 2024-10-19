use crate::token::Token;
use crate::object::Object;

pub trait Expr {
    // You can add method signatures here, for example:
    // fn accept(&self, visitor: &mut dyn Visitor) -> Result<(), String>;
}

// Binary expression
pub struct Binary {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Binary {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

// Grouping expression
pub struct Grouping {
    pub expression: Box<dyn Expr>,
}

impl Grouping {
    pub fn new(expression: Box<dyn Expr>) -> Self {
        Self { expression }
    }
}

// Literal expression
pub struct Literal {
    pub value: Object,
}

impl Literal {
    pub fn new(value: Object) -> Self {
        Self { value }
    }
}

// Unary expression
pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expr>) -> Self {
        Self { 
            operator, 
            right 
        }
    }
}
