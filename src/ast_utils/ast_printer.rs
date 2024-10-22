use crate::expression::{Unary, Binary, Visitor, LiteralValue, Literal, Grouping, Expr};

pub struct AstPrinter;
impl Visitor<String> for AstPrinter {
    
    fn visit_unary(&self, expr: &Unary) -> String {
        let c_expr = expr.clone();
        
        self.parenthesize(c_expr.operator.lexeme, &[&*c_expr.right])
    }
    fn visit_binary(&self, expr: &Binary) -> String {
        let c_expr = expr.clone();        
        self.parenthesize(c_expr.operator.lexeme, &[&*c_expr.left,&*c_expr.right])    
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
        return self.parenthesize("group".to_string(), &[&*expr.expression]);
    }
}

impl AstPrinter {
    
    pub fn new() -> AstPrinter {
        Self {}
    }
    
    pub fn print(&self, expr: Expr) -> String {
        return expr.accept(self);
    }
    
    fn parenthesize(&self, name: String, expression: &[&Expr]) -> String{
        let mut text: String = "(".to_string();
        text += &name;
        for expr  in expression {
            text += " ";
            text += &expr.accept(self)
        }
        text += ")";
        
        text
    }
    
}

