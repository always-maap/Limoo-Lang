#[cfg(test)]
mod lexer_test {
    use crate::{lexer::Lexer, token::Token};

    fn test_runner(input: &str, expected: &Vec<Token>) {
        let mut lexer = Lexer::new(&input);

        for test in expected.iter() {
            let token = lexer.next_token();
            assert_eq!(&token, test);
        }
    }

    #[test]
    fn test_let_tokens() {
        let test = "let five = 5;
                let ten = 10;
            ";

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
            Token::EOF,
        ];

        test_runner(test, &expected);
    }

    #[test]
    fn test_function_token() {
        let test = "let add = fn(x, y) {
                    x + y;
                 };
                 let result = add(five, ten);";

        let expected = vec![
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
            Token::EOF,
        ];

        test_runner(test, &expected);
    }

    #[test]
    fn test_if_tokens() {
        let test = "if (5 < 10) {
                    return true;
                 } else {
                    return false;
                 }
                ";

        let expected = vec![
            Token::IF,
            Token::LPAREN,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::BOOLEAN(true),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::BOOLEAN(false),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::EOF,
        ];

        test_runner(test, &expected);
    }

    #[test]
    fn test_operator_tokens() {
        let test = r#"
                 *!-/5;
                 && ||
                 5 < 10 > 5;
                 "#;

        let expected = vec![
            Token::ASTERISK,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::INT(5),
            Token::SEMICOLON,
            Token::AND,
            Token::OR,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::GT,
            Token::INT(5),
            Token::SEMICOLON,
            Token::EOF,
        ];

        test_runner(test, &expected);
    }

    #[test]
    fn test_int_tokens() {
        let test = r#"
                10 == 10;
                10 != 9;
                "#;

        let expected = vec![
            Token::INT(10),
            Token::EQ,
            Token::INT(10),
            Token::SEMICOLON,
            Token::INT(10),
            Token::NOT_EQ,
            Token::INT(9),
            Token::SEMICOLON,
            Token::EOF,
        ];

        test_runner(test, &expected);
    }

    #[test]
    fn test_string_tokens() {
        let test = r#"
                "foobar"
                "foo bar"
                "#;

        let expected = vec![
            Token::STRING("foobar".to_string()),
            Token::STRING("foo bar".to_string()),
            Token::EOF,
        ];

        test_runner(test, &expected);
    }

    #[test]
    fn test_single_line_comment_tokens() {
        let test = r#"
            // this is a comment
            let five = 5; // this is a inline comment
        "#;

        let expected = vec![
            Token::LET,
            Token::IDENT("five".to_string()),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            Token::EOF,
        ];

        test_runner(test, &expected);
    }

    #[test]
    fn test_multi_line_comment_tokens() {
        let test = r#"
            /* this is a 
             * multiline comment
             */
            let five = 5;
        "#;

        let expected = vec![
            Token::LET,
            Token::IDENT("five".to_string()),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            Token::EOF,
        ];

        test_runner(test, &expected);
    }
}
