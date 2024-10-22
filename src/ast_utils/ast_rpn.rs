use crate::expression::{Binary, Expr, Grouping, Literal, LiteralValue, Unary, Visitor};

pub struct AstRpn;
impl Visitor<String> for AstRpn {
    
    fn visit_unary(&self, expr: &Unary) -> String {
        return format!("{} {}", expr.right.accept(self), expr.operator.lexeme);
    }
    fn visit_binary(&self, expr: &Binary) -> String {
        return format!("{} {} {}", expr.left.accept(self), expr.right.accept(self), expr.operator.lexeme);    
    }
    fn visit_literal(&self, expr: &Literal) -> String {
       
       match &expr.value {
            LiteralValue::Nil => "nil".to_string(),
            LiteralValue::Number(n) => n.to_string(),
            LiteralValue::String(s) => s.clone(),
            LiteralValue::Boolean(b) => b.to_string(),
       }
    }
       
    fn visit_grouping(&self, expr: &Grouping) -> String {
        return expr.expression.accept(self);
    }
}

impl AstRpn {
    
    pub fn new() -> AstRpn {
        Self {}
    }
    
    pub fn print(&self, expr: Expr) -> String {
        return expr.accept(self);
    }
}

