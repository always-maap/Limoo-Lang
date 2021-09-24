pub enum Token {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT(String), // add, foobar, x, y, ...
    INT(String),   // 1343456

    // Operators
    ASSIGN, // "="
    Plus,   // "+"

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
