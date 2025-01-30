use crate::expression::Expr;
use crate::expression::Visitor;
use crate::LiteralValue;
use crate::Token;

pub struct AstPrinter;
impl Visitor<String> for AstPrinter {
    fn visit_unary(&self, _operator: &Token, right: &Expr, lexeme: &String) -> String {
        self.parenthesize(lexeme, &[right])
    }
    
    fn visit_binary(&self, left: &Expr, _operator: &Token, right: &Expr, lexeme: &String) -> String {
        self.parenthesize(lexeme, &[left, right])
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
        return self.parenthesize("group".to_string(), &[expr]);
    }
}

impl AstPrinter {
    pub fn new() -> AstPrinter {
        Self {}
    }

    pub fn print(&self, expr: Expr) -> String {
        return expr.accept(self);
    }

    fn parenthesize(&self, name: impl Into<String>, exprs: &[&Expr]) -> String {
        let mut text = format!("({}", name.into());

        for expr in exprs {
            text.push(' ');
            text.push_str(&expr.accept(self));
        }
        text.push(')');
        text
    }
}
