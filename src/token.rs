use core::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,

    // Identifiers & literals
    IDENT(String),  // add, foobar, x, y, ...
    INT(i32),       // 1343456
    BOOLEAN(bool),  // true, false
    STRING(String), // "foobar"

    // Operators
    ASSIGN,   // "="
    PLUS,     // "+"
    MINUS,    // "-"
    BANG,     // "!"
    ASTERISK, // "*"
    SLASH,    // "/"
    LT,       // "<"
    GT,       // ">"
    EQ,       // "=="
    NOT_EQ,   // "!="

    // Delimiters
    COMMA,     // ","
    SEMICOLON, // ";"
    LPAREN,    // "("
    RPAREN,    // ")"
    LBRACE,    // "{"
    RBRACE,    // "}"

    // Keywords
    FUNCTION, // "FUNCTION"
    LET,      // "LET"
    IF,       // "IF"
    ELSE,     // "ELSE"
    RETURN,   // "RETURN"
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::IDENT(id) => write!(f, "{}", id),
            Token::INT(i) => write!(f, "{}", i),
            Token::BOOLEAN(b) => write!(f, "{}", b),
            Token::STRING(s) => write!(f, "{}", s),
            Token::ASSIGN => write!(f, "="),
            Token::PLUS => write!(f, "+"),
            Token::MINUS => write!(f, "-"),
            Token::BANG => write!(f, "!"),
            Token::ASTERISK => write!(f, "*"),
            Token::SLASH => write!(f, "/"),
            Token::LT => write!(f, "<"),
            Token::GT => write!(f, ">"),
            Token::EQ => write!(f, "=="),
            Token::NOT_EQ => write!(f, "!="),
            Token::COMMA => write!(f, ","),
            Token::SEMICOLON => write!(f, ";"),
            Token::LPAREN => write!(f, "("),
            Token::RPAREN => write!(f, ")"),
            Token::LBRACE => write!(f, "{{"),
            Token::RBRACE => write!(f, "}}"),
            Token::FUNCTION => write!(f, "fn"),
            Token::LET => write!(f, "let"),
            Token::IF => write!(f, "if"),
            Token::ELSE => write!(f, "else"),
            Token::RETURN => write!(f, "return"),
            token => write!(f, "{:?}", token),
        }
    }
}
