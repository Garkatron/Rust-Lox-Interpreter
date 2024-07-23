use crate::token::Token;

enum Expr {
    Binary(Box<BinaryExpr>),
    // Other expressions...
}

struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl BinaryExpr {
    fn new(left: Expr, operator: Token, right: Expr) -> Self {
        BinaryExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}