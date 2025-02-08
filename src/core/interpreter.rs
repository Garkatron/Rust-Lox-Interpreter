use super::environment::Environment;
use super::expression::Expr;
use super::stmt::Stmt;
use super::token::Token;
use super::{expression::LiteralValue, runtime_error::RuntimeError, token_type::TokenType};
use super::{expression::Visitor as ExpressionVisitor, stmt::Visitor as StatementVisitor};
pub struct Interpreter {
    environment: Environment
}

impl ExpressionVisitor<LiteralValue> for Interpreter {
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Result<LiteralValue, RuntimeError> {
        let lit = self.evaluate(right)?;
        match operator.t_type {
            TokenType::MINUS => match lit {
                LiteralValue::Number(n) => Ok(LiteralValue::Number(-n)),
                _ => Err(RuntimeError::BadOperator(
                    operator.clone(),
                    "Operand must be a number.".to_string(),
                )),
            },
            TokenType::BANG => Ok(LiteralValue::Boolean(!self.is_truthy(&lit))),
            _ => Err(RuntimeError::BadOperator(
                operator.clone(),
                "Invalid unary operator.".to_string(),
            )),
        }
    }

    fn visit_binary(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<LiteralValue, RuntimeError> {
        let left_lit = self.evaluate(left)?;
        let right_lit = self.evaluate(right)?;

        match (operator.t_type.clone(), &left_lit, &right_lit) {
            (TokenType::PLUS, LiteralValue::Number(n1), LiteralValue::Number(n2)) => {
                Ok(LiteralValue::Number(n1 + n2))
            }
            (TokenType::PLUS, LiteralValue::String(s1), LiteralValue::String(s2)) => {
                Ok(LiteralValue::String(format!("{}{}", s1, s2)))
            }
            (TokenType::PLUS, LiteralValue::String(s1), LiteralValue::Number(n)) => {
                Ok(LiteralValue::String(format!("{}{}", s1, n)))
            }
            (TokenType::MINUS, LiteralValue::Number(n1), LiteralValue::Number(n2)) => {
                Ok(LiteralValue::Number(n1 - n2))
            }
            (TokenType::MINUS, LiteralValue::String(s1), LiteralValue::String(s2)) => {
                Ok(LiteralValue::String(s1.replacen(s2, "", 1)))
            }
            (TokenType::SLASH, LiteralValue::Number(n1), LiteralValue::Number(n2)) => {
                if *n2 == 0.0 {
                    return Err(RuntimeError::BadOperator(
                        operator.clone(),
                        "Division by zero.".to_string(),
                    ));
                }
                Ok(LiteralValue::Number(n1 / n2))
            }
            (TokenType::STAR, LiteralValue::Number(n1), LiteralValue::Number(n2)) => {
                Ok(LiteralValue::Number(n1 * n2))
            }
            (TokenType::GREATER, LiteralValue::Number(n1), LiteralValue::Number(n2)) => {
                Ok(LiteralValue::Boolean(n1 > n2))
            }
            (TokenType::GREATER_EQUAL, LiteralValue::Number(n1), LiteralValue::Number(n2)) => {
                Ok(LiteralValue::Boolean(n1 >= n2))
            }
            (TokenType::LESS, LiteralValue::Number(n1), LiteralValue::Number(n2)) => {
                Ok(LiteralValue::Boolean(n1 < n2))
            }
            (TokenType::LESS_EQUAL, LiteralValue::Number(n1), LiteralValue::Number(n2)) => {
                Ok(LiteralValue::Boolean(n1 <= n2))
            }
            (TokenType::BANG_EQUAL, _, _) => {
                Ok(LiteralValue::Boolean(!self.is_equal(&left_lit, &right_lit)))
            }
            (TokenType::EQUAL_EQUAL, _, _) => {
                Ok(LiteralValue::Boolean(self.is_equal(&left_lit, &right_lit)))
            }
            _ => Err(RuntimeError::BadOperator(
                operator.clone(),
                "Invalid binary operation.".to_string(),
            )),
        }
    }

    fn visit_literal(&mut self, value: &LiteralValue) -> Result<LiteralValue, RuntimeError> {
        Ok(value.clone())
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Result<LiteralValue, RuntimeError> {
        self.evaluate(expr)
    }

    fn visit_comma(&mut self, _: &Expr, right: &Expr) -> Result<LiteralValue, RuntimeError> {
        self.evaluate(right)
    }

    fn visit_ternary(
        &mut self,
        condition: &Expr,
        then_branch: &Expr,
        else_branch: &Expr,
    ) -> Result<LiteralValue, RuntimeError> {
        let condition_value = self.evaluate(condition)?;
    
        if self.is_truthy(&condition_value) {
            self.evaluate(then_branch) 
        } else {
            self.evaluate(else_branch)
        }
    }
    
    fn visit_variable(&mut self, name: &Token) -> Result<LiteralValue, RuntimeError> {
        Ok(self.environment.get(name)?)
    }
    
    fn visit_assing(&mut self, name: &Token, expr: &Expr) -> Result<LiteralValue, RuntimeError> {
        let value = self.mut_evaluate(expr)?;
        self.environment.assing(name, value.clone())?;
        Ok(value)
    }

    fn visit_logical(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<LiteralValue, RuntimeError> {
        let left_value = self.evaluate(left)?;
        if operator.t_type == TokenType::OR {
            if self.is_truthy(&left_value) {return Ok(left_value)}
        } else {
            if !self.is_truthy(&left_value) {return Ok(left_value)}

        }
        Ok(self.evaluate(right)?)
    }
}
impl StatementVisitor<()> for Interpreter {
    fn visit_print(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Print { expression } => {
                let value = self.evaluate(&expression)?;
                println!("{}", self.stringify(&value));
                Ok(())
            }
            _ => Err(RuntimeError::BadStatement("Expected Print".to_string())),
        }
    }

    fn visit_expression(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression { expression } => {
                let value = self.evaluate(&expression)?;
                println!("{}", self.stringify(&value));
                Ok(())
            }
            _ => Err(RuntimeError::BadStatement(
                "Expected expression".to_string(),
            )),
        }
    }
    fn visit_var(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        if let Stmt::Var { name, initializer } = stmt {
            let value = self.evaluate(&initializer)?;

            return self.environment.define(&name.lexeme, value);
            
        } else {
            Err(RuntimeError::BadStatement("Expected variable declaration".to_string()))
        }
    }
    fn visit_block(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Block { statements } => {
                self.execute_block(statements, Environment::new(Some(self.environment.clone())))
            }
            _ => {
                Err(RuntimeError::BadStatement("Expected block".to_string()))
            }
        }
    }
    fn visit_if(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::If { condition, then_branch, else_branch } => {
                let value = self.evaluate(condition)?;
                if self.is_truthy(&value) {
                    self.execute(&then_branch)
                } else if let Some(t_else_branch) = else_branch {
                    self.execute(t_else_branch)
                } else {
                    Err(RuntimeError::BadStatement("Expected if".to_string()))
                }
            }
            _ => {
                Err(RuntimeError::BadStatement("Expected if".to_string()))

            }
        }
    }
    
}

impl Interpreter {

    pub fn new() -> Self {
        Self {
            environment: Environment::new(None)
        }
    }

    fn execute_block(&mut self, statements: &[Stmt], env: Environment) -> Result<(), RuntimeError> {
        let prev_env = std::mem::replace(&mut self.environment, env);
    
        let result = statements.iter().try_for_each(|stmt| self.execute(stmt));
    
        self.environment = prev_env;
        result
    }
    
    fn evaluate(&mut self, expr: &Expr) -> Result<LiteralValue, RuntimeError> {
        expr.accept(self)
    }

    
    fn mut_evaluate(&mut self, expr: &Expr) -> Result<LiteralValue, RuntimeError> {
        expr.accept(self)
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in statements {
            self.execute(&statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        stmt.accept(self)?;
        Ok(())
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

    fn stringify(&self, lit: &LiteralValue) -> String {
        match lit {
            LiteralValue::Nil => "nil".to_string(),
            LiteralValue::Number(n) => n.to_string(),
            LiteralValue::String(s) => s.clone(),
            LiteralValue::Boolean(b) => b.to_string(),
        }
    }

    pub fn is_truthy(&self, value: &LiteralValue) -> bool {
        match value {
            LiteralValue::Boolean(b) => *b,
            LiteralValue::Nil => false,
            _ => true,
        }
    }
}
