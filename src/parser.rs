use crate::error_reporter::ErrorReporter;
use crate::parse_error::ParseError;
use crate::TokenType::*;
use crate::{expression::*, token::Token, token_type::TokenType};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,
    error_reporter: &'a mut ErrorReporter,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, error_reporter: &'a mut ErrorReporter) -> Self {
        Self {
            tokens,
            current: 0,
            error_reporter,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let result = self.expression();
        if result.is_err() {
            self.synchronize();
        }
        result
    }

    pub fn expression(&mut self) -> Result<Expr, ParseError> {
        self.comma()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparision()?;

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
            expr = Expr::Comma { left: Box::new(expr), right: Box::new(right) }
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
            self.consume(RIGHT_PAREN, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }

        Err(self.error(self.peek().clone(), "Expect expression."))
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

    fn error(&mut self, token: Token, message: &str) -> ParseError {
        self.error_reporter.token_error(&token, message);
        ParseError
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            match self.peek().t_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => self.advance(),
            };
        }
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(ttype) {
            Ok(self.advance())
        } else {
            Err(self.error(self.peek().clone(), message))
        }
    }
}
