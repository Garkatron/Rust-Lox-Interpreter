// https://craftinginterpreters.com/parsing-expressions.html#the-parser-class
use crate::{expression::Expr, token::Token, token_type::TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0
        }
    }

    fn expression(&self) -> Expr {
        self.equality()
    }

    fn equality(&self) -> Expr {
        let expr = self.comparison();

        while match(BANG_EQUAL, EQUAL_EQUAL) {
            let ope = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::Binary(expr, ope, right);
        }

        expr
    }

    fn match(&self, types: &[&TokenType]) -> bool {
        for ttype in types {
            if check(ttype) {
                self.advance();
                true
            }
        }
        false
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.isAtEnd() {
            false
        }
        self.peek().type == ttype
    }

    fn advance(&mut self) -> Token {
        if !self.isAtEnd() {
            self.current+=1;
        }
        self.previous()
    }

    fn isAtEnd(&self) -> bool {
        self.peek().type = EOF;
    
    }

    fn peek(&self) -> Token {
        self.tokens[current];
    }

    fn previous(&self) {
        self.tokens.get(current-1)
    }

    // https://craftinginterpreters.com/parsing-expressions.html#the-parser-class:~:text=isAtEnd()%20checks,binary%20operator%20nodes.

}