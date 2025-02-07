use crate::utils::colors::Color;

use super::expression::LiteralValue;
use super::token_type::TokenType::{self, *};
use super::{expression::Expr, parse_error::ParseError, stmt::Stmt, token::Token};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: vec![],
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements: Vec<Stmt> = vec![];

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    self.errors.push(e.clone());
                    self.synchronize();
                }
            }
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_tokens(&[VAR]) {
            return self.var_declaration();
        };

        self.statement()
    }
    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(IDENTIFIER, ParseError::ExpectedVariableName(0))?;

        let mut initializer = Expr::Literal {
            value: LiteralValue::Nil,
        }; // ! ALL VARS NOT INITIALIZED ARE NULL

        if self.match_tokens(&[EQUAL]) {
            initializer = self.expression()?;
        }

        self.consume(SEMICOLON, ParseError::ExpectedVariableDeclaration(0))?;

        Ok(Stmt::Var {
            name,
            initializer: Box::new(initializer),
        })
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_tokens(&[PRINT]) {
            return self.print_statement();
        }
        return self.expression_statement();
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value: Expr = self.expression()?;
        self.consume(SEMICOLON, ParseError::EspectSemicolonAfterValue(0))?;
        Ok(Stmt::Print {
            expression: Box::new(value),
        })
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.ternary()?;
        if self.match_tokens(&[EQUAL]) {
            let _equals = self.previous();
            let value = self.assignment()?;
            
            match expr {
                Expr::Variable { name } => {
                    return Ok(Expr::Assing { name, value: Box::new(value) })
                }
                _ => Color::ecprintln(&ParseError::InvalidAssignmentTarget(0).to_string(), Color::Red),
            }
        }
        Ok(expr)
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr: Expr = self.expression()?;
        self.consume(SEMICOLON, ParseError::EspectSemicolonAfterExpression(0))?;
        return Ok(Stmt::Expression {
            expression: Box::new(expr),
        });
    }

    fn ternary(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.comma()?;

        while self.match_tokens(&[QUESTION_MARK]) {
            let condition = expr.clone();
            let then_branch = self.comma()?;

            self.consume(COLON, ParseError::ExpectedTernaryBranch(0, 0))?;

            let else_branch = self.expression()?;

            expr = Expr::Ternary {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch: Box::new(else_branch),
            }
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.comparision()?;

        while self.match_tokens(&[BANG_EQUAL, EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparision()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comma(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;
        while self.match_tokens(&[COMMA]) {
            let right = self.equality()?;
            expr = Expr::Comma {
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn comparision(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_tokens(&[GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[PLUS, MINUS]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[STAR, SLASH]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_tokens(&[BANG, MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator: operator.clone(),
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_tokens(&[FALSE]) {
            return Ok(Expr::Literal {
                value: LiteralValue::Boolean(false),
            });
        }
        if self.match_tokens(&[TRUE]) {
            return Ok(Expr::Literal {
                value: LiteralValue::Boolean(true),
            });
        }
        if self.match_tokens(&[NIL]) {
            return Ok(Expr::Literal {
                value: LiteralValue::Nil,
            });
        }
        if self.match_tokens(&[NUMBER, STRING]) {
            return Ok(Expr::Literal {
                value: self.previous().literal.clone(),
            });
        }
        if self.match_tokens(&[LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(
                RIGHT_PAREN,
                ParseError::ExpectedRightParen(self.peek().line),
            )?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }
        if self.match_tokens(&[IDENTIFIER]) {
            return Ok(Expr::Variable { name: self.previous() })
        }

        Err(self.report_error(ParseError::InvalidExpression(
            format!(
                "Expected a valid expression, found: {:?}",
                self.peek().t_type
            ),
            self.peek().line,
        )))
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for ttype in types {
            if self.check(ttype.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().t_type == ttype
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().t_type, EOF)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn report_error(&mut self, error: ParseError) -> ParseError {
        self.errors.push(error.clone());
        Color::ecprintln(&format!("{}", error), Color::Red);
        error
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            match self.peek().t_type {
                CLASS | FUN | VAR | FOR | IF | WHILE | PRINT | RETURN => return,
                _ => self.advance(),
            };
        }
    }

    fn consume(&mut self, ttype: TokenType, message: ParseError) -> Result<Token, ParseError> {
        if self.check(ttype) {
            Ok(self.advance())
        } else {
            Err(self.report_error(message))
        }
    }
}
