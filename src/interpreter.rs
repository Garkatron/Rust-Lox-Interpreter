use crate::expression::{Expr, Visitor};
use crate::token_type::TokenType;
use crate::LiteralValue;
use crate::Token;
pub struct Interpreter;

// LAST https://craftinginterpreters.com/evaluating-expressions.html#detecting-runtime-errors

impl Visitor<LiteralValue> for Interpreter {
    fn visit_unary(&self, operator: &Token, right: &Expr) -> LiteralValue {
        let lit = self.evaluate(right);
        match operator.t_type {
            TokenType::MINUS => {

                match lit {
                    LiteralValue::Number(n) => LiteralValue::Number(-n),
                    _ => LiteralValue::Nil,
                }
            }
            TokenType::BANG => LiteralValue::Boolean(!self.is_truthy(lit)),
            _ => {
                return LiteralValue::Nil;
            }
        }
    }

    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> LiteralValue {
        let left_lit = self.evaluate(left);
        let right_lit = self.evaluate(right);

        match (operator.t_type.clone(), left_lit, right_lit) {
            (TokenType::PLUS, LiteralValue::Number(n), LiteralValue::Number(n2)) => {
                LiteralValue::Number(n + n2)
            }
            (TokenType::PLUS, LiteralValue::String(s), LiteralValue::String(s2)) => {
                LiteralValue::String(format!("{}{}", s, s2))
            }

            (TokenType::MINUS, LiteralValue::Number(n), LiteralValue::Number(n2)) => {
                LiteralValue::Number(n - n2)
            }

            (TokenType::SLASH, LiteralValue::Number(n), LiteralValue::Number(n2)) => {
                LiteralValue::Number(n / n2)
            }

            (TokenType::STAR, LiteralValue::Number(n), LiteralValue::Number(n2)) => {
                LiteralValue::Number(n * n2)
            }

            (TokenType::GREATER, LiteralValue::Number(n), LiteralValue::Number(n2)) => {
                LiteralValue::Boolean(n > n2)
            }

            (TokenType::GREATER_EQUAL, LiteralValue::Number(n), LiteralValue::Number(n2)) => {
                LiteralValue::Boolean(n >= n2)
            }

            (TokenType::LESS, LiteralValue::Number(n), LiteralValue::Number(n2)) => {
                LiteralValue::Boolean(n < n2)
            }

            (TokenType::LESS_EQUAL, LiteralValue::Number(n), LiteralValue::Number(n2)) => {
                LiteralValue::Boolean(n <= n2)
            }

            (
                TokenType::MINUS
                | TokenType::SLASH
                | TokenType::STAR
                | TokenType::GREATER
                | TokenType::GREATER_EQUAL
                | TokenType::LESS
                | TokenType::LESS_EQUAL,
                _,
                _,
            ) => LiteralValue::Nil,

            (TokenType::BANG_EQUAL, left, right) => {
                LiteralValue::Boolean(!self.is_equal(&left, &right))
            }
            (TokenType::EQUAL_EQUAL, left, right) => {
                LiteralValue::Boolean(self.is_equal(&left, &right))
            }
            _ => LiteralValue::Nil,
        }
    }

    fn visit_literal(&self, value: &LiteralValue) -> LiteralValue {
        value.clone()
    }

    fn visit_grouping(&self, expr: &Expr) -> LiteralValue {
        self.evaluate(expr)
    }

    fn visit_comma(&self, left: &Expr, right: &Expr) -> LiteralValue {}

    fn visit_ternary(
        &self,
        condition: &Expr,
        then_branch: &Expr,
        else_branch: &Expr,
    ) -> LiteralValue {
    }
}

impl Interpreter {
    pub fn evaluate(&self, expr: &Expr) -> LiteralValue {
        match expr.accept(self) {
            Ok(resultado) => resultado,
            Err(error) => panic!("Error al evaluar la expresión: {}", error),
        }
    }
 
    fn is_equal(&self, left: &LiteralValue, right: &LiteralValue) -> bool {
        match (left, right) {
            (LiteralValue::Nil, LiteralValue::Nil) => true,

            (LiteralValue::Nil, _) | (_, LiteralValue::Nil) => false,

            (LiteralValue::Number(n1), LiteralValue::Number(n2)) => n1 == n2,

            (LiteralValue::String(s1), LiteralValue::String(s2)) => s1 == s2,

            (LiteralValue::Boolean(b1), LiteralValue::Boolean(b2)) => b1 == b2,

            _ => false,
        }
    }

    pub fn is_truthy(&self, value: LiteralValue) -> bool {
        match value {
            LiteralValue::Boolean(b) => b,
            LiteralValue::Nil => false,
            _ => true,
        }
    }
}
