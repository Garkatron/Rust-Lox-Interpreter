use std::fmt::Binary;

use crate::{expr::expression::{self, Expr, LiteralValue}, token::Token, token_type::TokenType};

pub struct AstPrinter;


impl expression::Visitor<String> for AstPrinter {
    
    fn visit_unary(&self, expr: &expression::Unary) -> String {
        let c_expr = expr.clone();
        
        self.parenthesize(c_expr.operator.lexeme, &[&*c_expr.right])
    }
    fn visit_binary(&self, expr: &expression::Binary) -> String {
        let c_expr = expr.clone();        
        self.parenthesize(c_expr.operator.lexeme, &[&*c_expr.left,&*c_expr.right])    
    }
    fn visit_literal(&self, expr: &expression::Literal) -> String {
       
       match &expr.value {
            expression::LiteralValue::Nil => "nil".to_string(),
            expression::LiteralValue::Number(n) => n.to_string(),
            expression::LiteralValue::String(s) => s.clone(),
            expression::LiteralValue::Boolean(b) => b.to_string(),
       }
    }
       
    fn visit_grouping(&self, expr: &expression::Grouping) -> String {
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

