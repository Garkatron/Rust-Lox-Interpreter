use std::fmt;

use crate::core::syntax::{token::Token, token_type::TokenType};

#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedToken(Token, usize),
    MissingToken(Token, usize),
    InvalidExpression(String, usize),
    UnterminatedString(usize),
    DivisionByZero(usize),
    UndefinedVariable(String, usize),
    UnexpectedEOF(usize),
    ExpectedRightParen(usize),
    ExpectedTernaryBranch(usize, usize),
    MissingLeftOperand(usize),
    EspectSemicolonAfterValue(usize),
    EspectSemicolonAfterExpression(usize),
    ExpectedVariableName(usize),
    ExpectedVariableDeclaration(usize),
    InvalidAssignmentTarget(usize),
    ExpectedRightBraceAfterBlock(usize),
    ExpectedRightBraceAfterClassBody(usize),
    ExpectedLeftBraceAfterClassBody(usize),
    ExpectedSomeTokenTypeAfterSomething(TokenType, usize, String),
    ExpectedBreak(usize),
    TooManyArguments(usize),
    ExpectedIdentifier(usize, String),
    ExpectedParameterName(usize),
    ExpectClassName(usize),
    ExpectedPropertyNameAfterDot(usize),
    ExpectedSuperClassName(usize),
    ExpectDotAfterSuper(usize),
    ExpectSuperClassMethodName(usize)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(token, line) => {
                write!(f, "[PARSER]: Unexpected token '{}' at line {}", token.lexeme, line)
            }
            ParseError::MissingToken(token, line) => {
                write!(f, "[PARSER]: Expected token '{}' but it was missing at line {}", token.lexeme, line)
            }
            ParseError::InvalidExpression(expr, line) => {
                write!(f, "[PARSER]: Invalid expression: '{}' at line {}", expr, line)
            }
            ParseError::UnterminatedString(line) => {
                write!(f, "[PARSER]: Unterminated string literal at line {}", line)
            }
            ParseError::DivisionByZero(line) => {
                write!(f, "[PARSER]: Division by zero detected at line {}", line)
            }
            ParseError::UndefinedVariable(var_name, line) => {
                write!(f, "[PARSER]: Undefined variable '{}' at line {}", var_name, line)
            }
            ParseError::UnexpectedEOF(line) => {
                write!(f, "[PARSER]: Unexpected end of file at line {}", line)
            }
            ParseError::ExpectedRightParen(line) => {
                write!(f, "[PARSER]: Expect ')' after expression, at line {}", line)
            }
            ParseError::ExpectedTernaryBranch(line, expr_line) => {
                write!(f, "[PARSER]: Expect ':' after then branch of ternary expression at line {} (expression at line {})", line, expr_line)
            }
            ParseError::MissingLeftOperand(line) => {
                write!(f, "[PARSER]: Expect a left operand after an unary expression, at line: {}", line)
            }
            ParseError::EspectSemicolonAfterValue(line) => {
                write!(f, "[PARSER]: Expect ';' after value. At line: {}", line)
            }
            ParseError::EspectSemicolonAfterExpression(line) => {
                write!(f, "[PARSER]: Expect ';' after expression. At line: {}", line)
            }
            ParseError::ExpectedVariableName(line) => {
                write!(f, "[PARSER]: Expect variable name. At line: {}", line)
            }
            ParseError::ExpectedVariableDeclaration(line) => {
                write!(f, "[PARSER]: Expect variable declaration. At line: {}", line)
            }
            ParseError::InvalidAssignmentTarget(line) => {
                write!(f, "[PARSER]: Invalid assignment target. At line {}", line)
            }
            ParseError::ExpectedRightBraceAfterBlock(line) => {
                write!(f, "[PARSER]: Expect '}}' after block. At line {}", line)
            }
            ParseError::ExpectedSomeTokenTypeAfterSomething(tt, line, something) => {
                write!(f, "[PARSER]: Expect '{}' after {} statement at line {}", tt,  something, line)
            }
            ParseError::ExpectedBreak(line) => {
                write!(f, "[PARSER]: Expect break statement at line {}", line)
            }
            ParseError::TooManyArguments(line) => {
                write!(f, "[PARSER]: Too many arguments at line {}", line)
            }
            ParseError::ExpectedIdentifier(line, kind) => {
                write!(f, "[PARSER]: Expect {} name at line {}", line, kind)  

            }
            ParseError::ExpectedParameterName(line) => {
                write!(f, "[PARSER]: Expect parameter name at line {}", line)
            }
            ParseError::ExpectClassName(line) => {
                write!(f, "[PARSER]: Expect class name {}", line)
            }
            ParseError::ExpectedLeftBraceAfterClassBody(line) => {
                write!(f, "[PARSER]: Expect '{{' after class body. At line {}", line)
            }
            ParseError::ExpectedRightBraceAfterClassBody(line) => {
                write!(f, "[PARSER]: Expect '}}' after class body. At line {}", line)
            }
            ParseError::ExpectedPropertyNameAfterDot(line) => {
                write!(f, "[PARSER]: Expected property name after '.' at line {}", line)
            }
            ParseError::ExpectedSuperClassName(line) => {
                write!(f, "[PARSER]: Expected superclass name at line {}", line)
            }
            ParseError::ExpectSuperClassMethodName(line) => {
                write!(f, "[PARSER]: Expect superclass method name {}", line)
            }
            ParseError::ExpectDotAfterSuper(line) => {
                write!(f, "[PARSER]: Expect dot after super name {}", line)
            }
        }
    }
}

impl std::error::Error for ParseError {}

impl ParseError {
    pub fn to_string(&self) -> String {
        match self {
            ParseError::UnexpectedToken(token, line) => {
                format!("[PARSER]: Unexpected token '{}' at line {}", token.lexeme, line)
            }
            ParseError::MissingToken(token, line) => {
                format!("[PARSER]: Expected token '{}' but it was missing at line {}", token.lexeme, line)
            }
            ParseError::InvalidExpression(expr, line) => {
                format!("[PARSER]: Invalid expression: '{}' at line {}", expr, line)
            }
            ParseError::UnterminatedString(line) => {
                format!("[PARSER]: Unterminated string literal at line {}", line)
            }
            ParseError::DivisionByZero(line) => {
                format!("[PARSER]: Division by zero detected at line {}", line)
            }
            ParseError::UndefinedVariable(var_name, line) => {
                format!("[PARSER]: Undefined variable '{}' at line {}", var_name, line)
            }
            ParseError::UnexpectedEOF(line) => {
                format!("[PARSER]: Unexpected end of file at line {}", line)
            }
            ParseError::ExpectedRightParen(line) => {
                format!("[PARSER]: Expect ')' after expression, at line {}", line)
            }
            ParseError::ExpectedTernaryBranch(line, expr_line) => {
                format!("[PARSER]: Expect ':' after then branch of ternary expression at line {} (expression at line {})", line, expr_line)
            }
            ParseError::MissingLeftOperand(line) => {
                format!("[PARSER]: Expect a left operand after an unary expression, at line: {}", line)
            }
            ParseError::EspectSemicolonAfterValue(line) => {
                format!("[PARSER]: Expect ';' after value. At line: {}", line)
            }
            ParseError::EspectSemicolonAfterExpression(line) => {
                format!("[PARSER]: Expect ';' after expression. At line: {}", line)
            }
            ParseError::ExpectedVariableName(line) => {
                format!("[PARSER]: Expect variable name. At line: {}", line)
            }
            ParseError::ExpectedVariableDeclaration(line) => {
                format!("[PARSER]: Expect variable declaration. At line: {}", line)
            }
            ParseError::InvalidAssignmentTarget(line) => {
                format!("[PARSER]: Invalid assignment target. At line {}", line)
            }
            ParseError::ExpectedRightBraceAfterBlock(line) => {
                format!("[PARSER]: Expect '}}' after block. At line {}", line)
            }
            ParseError::ExpectedSomeTokenTypeAfterSomething(tt, line, something) => {
                format!("[PARSER]: Expect '{}' after {} statement at line {}", tt,  something, line)
            }
            ParseError::ExpectedBreak(line) => {
                format!("[PARSER]: Expect break statement at line {}", line)
            }
            ParseError::TooManyArguments(line) => {
                format!("[PARSER]: Too many arguments at line {}", line)  // Mensaje para TooManyArguments
            }
            ParseError::ExpectedIdentifier(line, kind) => {
                format!("[PARSER]: Expect {} name at line {}", line, kind)  
            }
            ParseError::ExpectedParameterName(line) => {
                format!("[PARSER]: Expect parameter name at line {}", line)
            }
            ParseError::ExpectedLeftBraceAfterClassBody(line) => {
                format!("[PARSER]: Expect '{{' after class body. At line {}", line)
            }
            ParseError::ExpectedRightBraceAfterClassBody(line) => {
                format!("[PARSER]: Expect '}}' after class body. At line {}", line)
            }
            ParseError::ExpectClassName(line) => {
                format!("[PARSER]: Expect '}}' after class body. At line {}", line)
            }
            ParseError::ExpectedPropertyNameAfterDot(line) => {
                format!("[PARSER]: Expected property name after '.' at line {}", line)
            }
            ParseError::ExpectedSuperClassName(line) => {
                format!("[PARSER]: Expected superclass name at line {}", line)
            }
            ParseError::ExpectSuperClassMethodName(line) => {
                format!("[PARSER]: Expect superclass method name {}", line)
            }
            ParseError::ExpectDotAfterSuper(line) => {
                format!("[PARSER]: Expect dot after super name {}", line)
            }
        }
    }
}

impl From<(String, usize)> for ParseError {
    fn from(message: (String, usize)) -> Self {
        ParseError::InvalidExpression(message.0, message.1)
    }
}
