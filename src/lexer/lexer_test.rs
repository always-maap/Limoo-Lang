use crate::{lexer::Lexer, token::Token};

#[test]
fn test_next_token() {
    let input = "=+(){},;";
    let tests = vec![
        Token::ASSIGN,
        Token::PLUS,
        Token::LPAREN,
        Token::RPAREN,
        Token::LBRACE,
        Token::RBRACE,
        Token::COMMA,
        Token::SEMICOLON,
        Token::EOF,
    ];

    let mut lexer = Lexer::new(input);
    for test in tests.iter() {
        let token = lexer.next_token();
        assert_eq!(&token, test);
    }
}
