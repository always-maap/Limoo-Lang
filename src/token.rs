#[derive(Debug, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT(String), // add, foobar, x, y, ...
    INT(i32),      // 1343456

    // Operators
    ASSIGN, // "="
    PLUS,   // "+"

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
}
