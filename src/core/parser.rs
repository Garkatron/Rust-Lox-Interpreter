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
        if self.match_tokens(&[FUN]) {
            return self.function("function");
        }
        if self.match_tokens(&[VAR]) {
            return self.var_declaration();
        };

        self.statement()
    }
    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(
            IDENTIFIER,
            ParseError::ExpectedVariableName(self.peek().line),
        )?;

        let mut initializer = Expr::Literal {
            value: LiteralValue::Nil,
        }; // ! ALL VARS NOT INITIALIZED ARE NULL

        if self.match_tokens(&[EQUAL]) {
            initializer = self.expression()?;
        }

        self.consume(
            SEMICOLON,
            ParseError::ExpectedVariableDeclaration(self.peek().line),
        )?;

        Ok(Stmt::Var {
            name,
            initializer: *Box::new(initializer),
        })
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, ParseError> {
        let name = self.consume(
            IDENTIFIER,
            ParseError::ExpectedIdentifier(self.peek().line, kind.to_string()),
        )?;

        self.consume(
            LEFT_PAREN,
            ParseError::ExpectedSomeTokenTypeAfterSomething(
                LEFT_PAREN,
                self.peek().line,
                "function".to_string(),
            ),
        )?;

        let mut params = vec![];

        if !self.check(RIGHT_PAREN) {
            loop {
                if params.len() >= 255 {
                    let _ = self.report_error(ParseError::TooManyArguments(self.peek().line));
                }

                params.push(self.consume(
                    IDENTIFIER,
                    ParseError::ExpectedParameterName(self.peek().line),
                )?);

                if !self.match_tokens(&[COMMA]) {
                    break;
                } // Continue if has commas
            }
        }
        self.consume(
            RIGHT_PAREN,
            ParseError::ExpectedSomeTokenTypeAfterSomething(
                RIGHT_PAREN,
                self.peek().line,
                "function".to_string(),
            ),
        )?;

        self.consume(
            LEFT_BRACE,
            ParseError::ExpectedSomeTokenTypeAfterSomething(
                LEFT_BRACE,
                self.peek().line,
                "function".to_string(),
            ),
        )?;

        let body = self.block()?;
        return Ok(Stmt::Function {
            token: name,
            params,
            body,
        });
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_tokens(&[FOR]) {
            return self.for_statement();
        }
        if self.match_tokens(&[IF]) {
            return self.if_statement();
        }
        if self.match_tokens(&[PRINT]) {
            return self.print_statement();
        }
        if self.match_tokens(&[WHILE]) {
            return self.while_statement();
        }
        if self.match_tokens(&[LOOP]) {
            return self.loop_statement();
        }
        if self.match_tokens(&[BREAK]) {
            return self.break_statement();
        }

        if self.match_tokens(&[LEFT_BRACE]) {
            return Ok(Stmt::Block {
                statements: self.block()?,
            });
        }

        return self.expression_statement();
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        // (
        self.consume(
            LEFT_PAREN,
            ParseError::ExpectedSomeTokenTypeAfterSomething(
                LEFT_PAREN,
                self.peek().line,
                "For".to_string(),
            ),
        )?;
        let initializer;
        if self.match_tokens(&[SEMICOLON]) {
            // ; No initializer
            initializer = Some(Stmt::Expression {
                expression: Expr::Literal {
                    value: LiteralValue::Nil,
                },
            });
        } else if self.match_tokens(&[VAR]) {
            // var a = ?
            initializer = Some(self.var_declaration()?);
        } else {
            // expr
            initializer = Some(self.expression_statement()?);
        }

        let mut condition = None;
        if !self.check(SEMICOLON) {
            // expr
            condition = Some(self.expression()?);
        }

        // ;
        self.consume(
            SEMICOLON,
            ParseError::ExpectedSomeTokenTypeAfterSomething(
                SEMICOLON,
                self.peek().line,
                "For".to_string(),
            ),
        )?;

        let increment;
        if !self.check(RIGHT_PAREN) {
            increment = Some(self.expression()?);
        } else {
            // Incremento vacío
            increment = Some(Expr::Literal {
                value: LiteralValue::Nil,
            });
        }

        self.consume(
            RIGHT_PAREN,
            ParseError::ExpectedSomeTokenTypeAfterSomething(
                RIGHT_PAREN,
                self.peek().line,
                "For".to_string(),
            ),
        )?;

        let mut body = self.statement()?;

        if let Some(inc) = increment {
            body = Stmt::Block {
                statements: vec![body, Stmt::Expression { expression: inc }],
            };
        }

        body = Stmt::While {
            condition: condition.unwrap_or(Expr::Literal {
                value: LiteralValue::Boolean(true),
            }),
            body: Box::new(body),
            else_branch: None,
        };

        if let Some(ini) = initializer {
            body = Stmt::Block {
                statements: vec![ini, body],
            };
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(
            LEFT_PAREN,
            ParseError::ExpectedSomeTokenTypeAfterSomething(
                LEFT_PAREN,
                self.peek().line,
                "While".to_string(),
            ),
        )?;

        let condition = self.expression()?;

        self.consume(
            RIGHT_PAREN,
            ParseError::ExpectedSomeTokenTypeAfterSomething(
                RIGHT_PAREN,
                self.peek().line,
                "While".to_string(),
            ),
        )?;

        let body = self.statement()?;

        let mut else_branch = None;

        if self.match_tokens(&[ELSE]) {
            else_branch = Some(Box::new(self.statement()?));
        }

        return Ok(Stmt::While {
            condition,
            body: Box::new(body),
            else_branch,
        });
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(
            LEFT_PAREN,
            ParseError::ExpectedSomeTokenTypeAfterSomething(
                LEFT_PAREN,
                self.peek().line,
                "If".to_string(),
            ),
        )?;

        let condition = self.expression()?;

        self.consume(
            RIGHT_PAREN,
            ParseError::ExpectedSomeTokenTypeAfterSomething(
                RIGHT_PAREN,
                self.peek().line,
                "If".to_string(),
            ),
        )?;

        let then_branch = self.statement()?;
        let mut else_branch = None;

        if self.match_tokens(&[ELSE]) {
            else_branch = Some(Box::new(self.statement()?));
        }

        return Ok(Stmt::If {
            condition: *Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        });
    }

    fn loop_statement(&mut self) -> Result<Stmt, ParseError> {
        let body = self.statement()?;

        Ok(Stmt::Loop {
            body: Box::new(body),
        })
    }

    fn break_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(
            SEMICOLON,
            ParseError::ExpectedSomeTokenTypeAfterSomething(
                SEMICOLON,
                self.peek().line,
                "break".to_string(),
            ),
        )?;
        Ok(Stmt::Break {})
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        while !self.check(RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(
            RIGHT_BRACE,
            ParseError::ExpectedRightBraceAfterBlock(self.peek().line),
        )?;
        return Ok(statements);
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value: Expr = self.expression()?;
        self.consume(
            SEMICOLON,
            ParseError::EspectSemicolonAfterValue(self.peek().line),
        )?;
        Ok(Stmt::Print {
            expression: *Box::new(value),
        })
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;
        if self.match_tokens(&[EQUAL]) {
            let _equals = self.previous();
            let value = self.assignment()?;

            match expr {
                Expr::Variable { name } => {
                    return Ok(Expr::Assing {
                        name,
                        value: Box::new(value),
                    })
                }
                _ => Color::ecprintln(
                    &ParseError::InvalidAssignmentTarget(self.current).to_string(),
                    Color::Red,
                ),
            }
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;
        while self.match_tokens(&[OR]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.ternary()?;
        while self.match_tokens(&[AND]) {
            let operator = self.previous();
            let right = self.ternary()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr: Expr = self.expression()?;
        self.consume(
            SEMICOLON,
            ParseError::EspectSemicolonAfterExpression(self.peek().line),
        )?;
        return Ok(Stmt::Expression {
            expression: *Box::new(expr),
        });
    }

    fn ternary(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.equality()?;

        while self.match_tokens(&[QUESTION_MARK]) {
            let condition = expr.clone();
            let then_branch = self.equality()?;

            self.consume(
                COLON,
                ParseError::ExpectedTernaryBranch(self.peek().line, 0),
            )?;

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

    /*fn comma(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;
        while self.match_tokens(&[COMMA]) {
            let right = self.equality()?;
            expr = Expr::Comma {
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }*/

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

        self.call()
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;
        loop {
            if self.match_tokens(&[LEFT_PAREN]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut arguments = vec![];
        if !self.check(RIGHT_PAREN) {
            loop {
                if arguments.len() >= 255 {
                    Color::ecprintln(
                        &ParseError::TooManyArguments(self.peek().line).to_string(),
                        Color::Red,
                    );
                }
                arguments.push(self.expression()?);
                if !self.match_tokens(&[COMMA]) {
                    break;
                }
            }
        }        
        let paren = self.consume(
            RIGHT_PAREN,
            ParseError::ExpectedSomeTokenTypeAfterSomething(
                RIGHT_PAREN,
                self.peek().line,
                "call function".to_string(),
            ),
        )?;

        return Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        });
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
            return Ok(Expr::Variable {
                name: self.previous(),
            });
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
