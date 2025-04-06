use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Eq, Hash)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR, QUESTION_MARK, COLON,

    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE, LOOP, BREAK,
    STATIC, PUB,

    EOF
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token_str = match self {
            TokenType::LEFT_PAREN => "left_paren",
            TokenType::RIGHT_PAREN => "right_paren",
            TokenType::LEFT_BRACE => "left_brace",
            TokenType::RIGHT_BRACE => "right_brace",
            TokenType::COMMA => "comma",
            TokenType::DOT => "dot",
            TokenType::MINUS => "minus",
            TokenType::PLUS => "plus",
            TokenType::SEMICOLON => "semicolon",
            TokenType::SLASH => "slash",
            TokenType::STAR => "star",
            TokenType::QUESTION_MARK => "question_mark",
            TokenType::COLON => "colon",
            TokenType::BANG => "bang",
            TokenType::BANG_EQUAL => "bang_equal",
            TokenType::EQUAL => "equal",
            TokenType::EQUAL_EQUAL => "equal_equal",
            TokenType::GREATER => "greater",
            TokenType::GREATER_EQUAL => "greater_equal",
            TokenType::LESS => "less",
            TokenType::LESS_EQUAL => "less_equal",
            TokenType::IDENTIFIER => "identifier",
            TokenType::STRING => "string",
            TokenType::NUMBER => "number",
            TokenType::AND => "and",
            TokenType::CLASS => "class",
            TokenType::ELSE => "else",
            TokenType::FALSE => "false",
            TokenType::FN => "fn",
            TokenType::FOR => "for",
            TokenType::IF => "if",
            TokenType::NIL => "nil",
            TokenType::OR => "or",
            TokenType::PRINT => "print",
            TokenType::RETURN => "return",
            TokenType::SUPER => "super",
            TokenType::THIS => "this",
            TokenType::TRUE => "true",
            TokenType::VAR => "var",
            TokenType::WHILE => "while",
            TokenType::EOF => "eof",
            TokenType::LOOP => "loop",
            TokenType::BREAK => "break",
            TokenType::PUB => "pub",
            TokenType::STATIC => "static"
        };
        write!(f, "{}", token_str)
    }
}

impl TokenType {
    pub fn to_string(&self) -> String {
        match self {
            TokenType::LEFT_PAREN => "left_paren".to_string(),
            TokenType::RIGHT_PAREN => "right_paren".to_string(),
            TokenType::LEFT_BRACE => "left_brace".to_string(),
            TokenType::RIGHT_BRACE => "right_brace".to_string(),
            TokenType::COMMA => "comma".to_string(),
            TokenType::DOT => "dot".to_string(),
            TokenType::MINUS => "minus".to_string(),
            TokenType::PLUS => "plus".to_string(),
            TokenType::SEMICOLON => "semicolon".to_string(),
            TokenType::SLASH => "slash".to_string(),
            TokenType::STAR => "star".to_string(),
            TokenType::QUESTION_MARK => "question_mark".to_string(),
            TokenType::COLON => "colon".to_string(),
            TokenType::BANG => "bang".to_string(),
            TokenType::BANG_EQUAL => "bang_equal".to_string(),
            TokenType::EQUAL => "equal".to_string(),
            TokenType::EQUAL_EQUAL => "equal_equal".to_string(),
            TokenType::GREATER => "greater".to_string(),
            TokenType::GREATER_EQUAL => "greater_equal".to_string(),
            TokenType::LESS => "less".to_string(),
            TokenType::LESS_EQUAL => "less_equal".to_string(),
            TokenType::IDENTIFIER => "identifier".to_string(),
            TokenType::STRING => "string".to_string(),
            TokenType::NUMBER => "number".to_string(),
            TokenType::AND => "and".to_string(),
            TokenType::CLASS => "class".to_string(),
            TokenType::ELSE => "else".to_string(),
            TokenType::FALSE => "false".to_string(),
            TokenType::FN => "fun".to_string(),
            TokenType::FOR => "for".to_string(),
            TokenType::IF => "if".to_string(),
            TokenType::NIL => "nil".to_string(),
            TokenType::OR => "or".to_string(),
            TokenType::PRINT => "print".to_string(),
            TokenType::RETURN => "return".to_string(),
            TokenType::SUPER => "super".to_string(),
            TokenType::THIS => "this".to_string(),
            TokenType::TRUE => "true".to_string(),
            TokenType::VAR => "var".to_string(),
            TokenType::WHILE => "while".to_string(),
            TokenType::EOF => "eof".to_string(),
            TokenType::LOOP => "loop".to_string(),
            TokenType::BREAK => "break".to_string(),
            TokenType::PUB => "pub".to_string(),
            TokenType::STATIC => "static".to_string()
        }
    }
}
