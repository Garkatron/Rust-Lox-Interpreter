// Reverse Polish Notation

use crate::expression::{Expr, LiteralValue, Visitor};
use crate::runtime_error::RuntimeError;

pub struct AstRpn;

impl Visitor<String> for AstRpn {
    fn visit_unary(&self, operator: &crate::token::Token, right: &Expr) -> Result<String, RuntimeError> {
        Ok(format!("{} {}", right.accept(self)?, &operator.lexeme))
    }

    fn visit_binary(&self, left: &Expr, operator: &crate::token::Token, right: &Expr) -> Result<String, RuntimeError> {
        Ok(format!("{} {} {}", left.accept(self)?, right.accept(self)?, &operator.lexeme))
    }

    fn visit_literal(&self, value: &LiteralValue) -> Result<String, RuntimeError> {
        Ok(match value {
            LiteralValue::Nil => "nil".to_string(),
            LiteralValue::Number(n) => n.to_string(),
            LiteralValue::String(s) => s.clone(),
            LiteralValue::Boolean(b) => b.to_string(),
        })
    }

    fn visit_grouping(&self, expression: &Expr) -> Result<String, RuntimeError> {
        expression.accept(self)
    }

    fn visit_comma(&self, left: &Expr, right: &Expr) -> Result<String, RuntimeError> {
        Ok(format!("{} {}", left.accept(self)?, right.accept(self)?))
    }

    fn visit_ternary(&self, condition: &Expr, then_branch: &Expr, else_branch: &Expr) -> Result<String, RuntimeError> {
        Ok(format!("({}) ? {} : {}", condition.accept(self)?, then_branch.accept(self)?, else_branch.accept(self)?))
    }
}

impl AstRpn {
    pub fn new() -> AstRpn {
        Self {}
    }

    pub fn print(&self, expr: Expr) -> String {
        expr.accept(self).unwrap_or_else(|err| format!("error: {:?}", err))
    }
}
