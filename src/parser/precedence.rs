use crate::token::Token;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    LOWEST,
    EQUALS,      // '==' or '!='
    LESSGREATER, // '>' or '<'
    SUM,         // '+' or '-'
    PRODUCT,     // '*' or '/'
    PREFIX,      // '-x' or '!x'
    CALL,        // 'myFunc(x)'
}

pub fn token_to_precedence(token: &Token) -> Precedence {
    match token {
        Token::LT | Token::GT => Precedence::LESSGREATER,
        Token::EQ | Token::NOT_EQ => Precedence::EQUALS,
        Token::PLUS | Token::MINUS => Precedence::SUM,
        Token::SLASH | Token::ASTERISK => Precedence::PRODUCT,
        Token::LPAREN => Precedence::CALL,
        _ => Precedence::LOWEST,
    }
}
