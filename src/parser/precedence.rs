use crate::token::Token;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    LOWEST,
    ASSIGN,      // =
    EQUALS,      // '==' or '!='
    LOGICAL,     // '&&' or '||'
    LESSGREATER, // '>' or '<'
    SUM,         // '+' or '-'
    PRODUCT,     // '*' or '/'
    PREFIX,      // '-x' or '!x'
    CALL,        // 'myFunc(x)'
    INDEX,       // 'myArray[0]'
}

pub fn token_to_precedence(token: &Token) -> Precedence {
    match token {
        Token::LT | Token::GT => Precedence::LESSGREATER,
        Token::EQ | Token::NOT_EQ => Precedence::EQUALS,
        Token::ASSIGN => Precedence::ASSIGN,
        Token::PLUS | Token::MINUS => Precedence::SUM,
        Token::SLASH | Token::ASTERISK => Precedence::PRODUCT,
        Token::AND | Token::OR => Precedence::LOGICAL,
        Token::LPAREN => Precedence::CALL,
        Token::LBRACKET => Precedence::INDEX,
        _ => Precedence::LOWEST,
    }
}
