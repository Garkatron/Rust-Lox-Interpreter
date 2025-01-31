use crate::expression::{Expr, Visitor};
use crate::LiteralValue;
use crate::Token;

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_unary(&self, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[right])
    }

    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[left, right])
    }

    fn visit_literal(&self, value: &LiteralValue) -> String {
        match value {
            LiteralValue::Nil => "nil".to_string(),
            LiteralValue::Number(n) => n.to_string(),
            LiteralValue::String(s) => s.clone(),
            LiteralValue::Boolean(b) => b.to_string(),
        }
    }

    fn visit_grouping(&self, expr: &Expr) -> String {
        self.parenthesize("group", &[expr])
    }

    fn visit_comma(&self, left: &Expr, right: &Expr) -> String {
        self.parenthesize("comma", &[left, right])
    }

    fn visit_ternary(&self, condition: &Expr, then_branch: &Expr, else_branch: &Expr) -> String {
        self.parenthesize("ternary", &[condition, then_branch, else_branch])
    }
}

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut text = format!("({}", name);

        for expr in exprs {
            text.push(' ');
            text.push_str(&expr.accept(self));
        }
        text.push(')');
        text
    }
}