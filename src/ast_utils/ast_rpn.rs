// Reverse Polish Notation

use crate::expression::{Expr, LiteralValue, Visitor};

pub struct AstRpn;

impl Visitor<String> for AstRpn {
    fn visit_unary(&self, _operator: &crate::token::Token, right: &Expr, lexeme: &String) -> String {
        format!("{} {}", right.accept(self), lexeme)
    }

    fn visit_binary(&self, left: &Expr, _operator: &crate::token::Token, right: &Expr, lexeme: &String) -> String {
        format!("{} {} {}", left.accept(self), right.accept(self), lexeme)
    }

    fn visit_literal(&self, value: &LiteralValue) -> String {
        match value {
            LiteralValue::Nil => "nil".to_string(),
            LiteralValue::Number(n) => n.to_string(),
            LiteralValue::String(s) => s.clone(),
            LiteralValue::Boolean(b) => b.to_string(),
        }
    }

    fn visit_grouping(&self, expression: &Expr) -> String {
        expression.accept(self)
    }
}

impl AstRpn {
    pub fn new() -> AstRpn {
        Self {}
    }

    pub fn print(&self, expr: Expr) -> String {
        expr.accept(self)
    }
}
