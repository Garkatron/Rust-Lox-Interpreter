use crate::token::Token;

#[derive(Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
        lexeme: String,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
        lexeme: String,
    },
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub trait Visitor<R> {
    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr, lexeme: &String) -> R;
    fn visit_grouping(&self, expression: &Expr) -> R;
    fn visit_literal(&self, value: &LiteralValue) -> R;
    fn visit_unary(&self, operator: &Token, right: &Expr, lexeme: &String) -> R;
}

impl Expr {
    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        match self {
            Expr::Binary { left, operator, right, lexeme } => {
                visitor.visit_binary(left, operator, right, lexeme)
            }
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Literal { value } => visitor.visit_literal(value),
            Expr::Unary { operator, right, lexeme } => {
                visitor.visit_unary(operator, right, lexeme)
            }
        }
    }
}
