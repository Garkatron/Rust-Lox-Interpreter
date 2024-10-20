use crate::expr::expression::{self, Expr};

pub struct AstPrinter;


impl expression::Visitor<String> for AstPrinter {
    
    fn visit_unary(&self, expr: &expression::Unary) -> String {
        return self.parenthesize(expr.operator.lexeme, expr.left, expr.right);
    }
    fn visit_binary(&self, expr: &expression::Binary) -> String {
    
    }
    fn visit_literal(&self, expr: &expression::Literal) -> String {
    
    }
    fn visit_grouping(&self, expr: &expression::Grouping) -> String {
    
    }
}

impl AstPrinter {
    fn print(&self, expr: Expr) -> String {
        return expr.accept(self);
    }
    
    fn parenthesize(&self, name: String, expression: &[Expr]) {
        let mut text: String = "(".to_string();
        for expr  in expression {
            expr.accept(self)
        }
    }
    
}