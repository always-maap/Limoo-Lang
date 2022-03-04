#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT(String), // add, foobar, x, y, ...
    INT(i32),      // 1343456

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

    LPAREN, // "("
    RPAREN, // ")"
    LBRACE, // "{"
    RBRACE, // "}"

    // Keywords
    FUNCTION, // "FUNCTION"
    LET,      // "LET"
    TRUE,     // "TRUE"
    FALSE,    // "FALSE",
    IF,       // "IF"
    ELSE,     // "ELSE"
    RETURN,   // "RETURN"
}
