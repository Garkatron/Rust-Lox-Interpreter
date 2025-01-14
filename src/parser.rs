use crate::{expression::{*}, token::Token, token_type::TokenType};
use crate::TokenType::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparision();

        while self.match_tokens(&[BANG_EQUAL, EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparision();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
                lexeme: operator.lexeme.clone(),
            };
        }

        expr
    }

    fn comparision(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_tokens(&[GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
                lexeme: operator.lexeme.clone(),
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_tokens(&[PLUS, MINUS]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
                lexeme: operator.lexeme.clone(),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_tokens(&[STAR, SLASH]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
                lexeme: operator.lexeme.clone(),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_tokens(&[BANG, MINUS]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary {
                operator: operator.clone(),
                right: Box::new(right),
                lexeme: operator.lexeme,
            };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_tokens(&[FALSE]) {
            return Expr::Literal {
                value: LiteralValue::Boolean(false),
            };
        }
        if self.match_tokens(&[TRUE]) {
            return Expr::Literal {
                value: LiteralValue::Boolean(true),
            };
        }
        if self.match_tokens(&[NIL]) {
            return Expr::Literal {
                value: LiteralValue::Nil,
            };
        }
        if self.match_tokens(&[NUMBER, STRING]) {
            return Expr::Literal {
                value: self.previous().literal.clone(),
            };
        }
        if self.match_tokens(&[LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(RIGHT_PAREN, "Expect ')' after expression.");
            return Expr::Grouping {
                expression: Box::new(expr),
            };
        }

        panic!("Unexpected token: {}", self.peek().lexeme);
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
        self.peek().t_type == EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Token {
        if self.check(ttype) {
            return self.advance();
        }
        panic!("{}", message);
    }
}
