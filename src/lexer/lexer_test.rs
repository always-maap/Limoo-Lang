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

#[test]
fn test_next_token_2() {
    let input = "let five = 5; \
                 let ten = 10; \
                 let add = fn(x, y) { \
                    x + y; \
                 }; \
                 let result = add(five, ten); \
                 !-/*5; \
                 5 < 10 > 5; \

                 if (5 < 10) { \
                    return true; \
                 } else { \
                    return false; \
                 } \

                 10 == 10; \
                 10 != 9;
                 ";

    let mut lexer = Lexer::new(&input);

    let expected = vec![
        Token::LET,
        Token::IDENT("five".to_string()),
        Token::ASSIGN,
        Token::INT(5),
        Token::SEMICOLON,
        Token::LET,
        Token::IDENT("ten".to_string()),
        Token::ASSIGN,
        Token::INT(10),
        Token::SEMICOLON,
        Token::LET,
        Token::IDENT("add".to_string()),
        Token::ASSIGN,
        Token::FUNCTION,
        Token::LPAREN,
        Token::IDENT("x".to_string()),
        Token::COMMA,
        Token::IDENT("y".to_string()),
        Token::RPAREN,
        Token::LBRACE,
        Token::IDENT("x".to_string()),
        Token::PLUS,
        Token::IDENT("y".to_string()),
        Token::SEMICOLON,
        Token::RBRACE,
        Token::SEMICOLON,
        Token::LET,
        Token::IDENT("result".to_string()),
        Token::ASSIGN,
        Token::IDENT("add".to_string()),
        Token::LPAREN,
        Token::IDENT("five".to_string()),
        Token::COMMA,
        Token::IDENT("ten".to_string()),
        Token::RPAREN,
        Token::SEMICOLON,
        Token::BANG,
        Token::MINUS,
        Token::SLASH,
        Token::ASTERISK,
        Token::INT(5),
        Token::SEMICOLON,
        Token::INT(5),
        Token::LT,
        Token::INT(10),
        Token::GT,
        Token::INT(5),
        Token::SEMICOLON,
        Token::IF,
        Token::LPAREN,
        Token::INT(5),
        Token::LT,
        Token::INT(10),
        Token::RPAREN,
        Token::LBRACE,
        Token::RETURN,
        Token::TRUE,
        Token::SEMICOLON,
        Token::RBRACE,
        Token::ELSE,
        Token::LBRACE,
        Token::RETURN,
        Token::FALSE,
        Token::SEMICOLON,
        Token::RBRACE,
    ];

    for test in expected.iter() {
        let token = lexer.next_token();
        assert_eq!(&token, test);
    }
}
