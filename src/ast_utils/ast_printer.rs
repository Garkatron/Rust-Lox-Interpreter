use crate::expression::Expr;
use crate::expression::Visitor;
use crate::LiteralValue;

pub struct AstPrinter;
impl Visitor<String> for AstPrinter {
    
    fn visit_unary(&self, expr: &Expr) -> String {
        let c_expr = expr.clone();
        
        self.parenthesize(c_expr.operator.lexeme, &[&*c_expr.right])
    }
    fn visit_binary(&self, expr: &Expr) -> String {
        let c_expr = expr.clone();        
        self.parenthesize(c_expr.operator.lexeme, &[&*c_expr.left,&*c_expr.right])    
    }
    fn visit_literal(&self, expr: &Expr) -> String {
       
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

    fn parenthesize(&self, name: impl Into<String>, expression: &[&Expr]) -> String {
        let mut text: String = "(".to_string();
        let namestr: String = name.into();
        text += &namestr;
        for expr in expression {
            text += " ";
            text += &expr.accept(self); 
        }
        text += ")";
        text
    }
    
    
}

