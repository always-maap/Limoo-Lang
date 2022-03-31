use crate::token::Token;

mod lexer_test;

#[derive(Debug, Default)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.chars().collect::<Vec<char>>(),
            ..Default::default()
        };

        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        let token: Token;

        self.skip_whitespace();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token = Token::EQ
                } else {
                    token = Token::ASSIGN
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token = Token::NOT_EQ
                } else {
                    token = Token::BANG
                }
            }
            ';' => token = Token::SEMICOLON,
            '(' => token = Token::LPAREN,
            ')' => token = Token::RPAREN,
            ',' => token = Token::COMMA,
            '+' => token = Token::PLUS,
            '-' => token = Token::MINUS,
            '/' => token = Token::SLASH,
            '*' => token = Token::ASTERISK,
            '<' => token = Token::LT,
            '>' => token = Token::GT,
            '{' => token = Token::LBRACE,
            '}' => token = Token::RBRACE,
            '\0' => token = Token::EOF,
            ch => {
                if ch.is_alphabetic() || ch == '_' {
                    let idenfifier = self.read_identifier();
                    return match idenfifier.as_str() {
                        "let" => Token::LET,
                        "fn" => Token::FUNCTION,
                        "true" => Token::BOOLEAN(true),
                        "false" => Token::BOOLEAN(false),
                        "if" => Token::IF,
                        "else" => Token::ELSE,
                        "return" => Token::RETURN,
                        _ => Token::IDENT(idenfifier),
                    };
                } else if ch.is_digit(10) {
                    let num = self.read_number();
                    return Token::INT(num);
                } else {
                    token = Token::ILLEGAL;
                }
            }
        }

        self.read_char();
        return token;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let start_index = self.position;

        while self.ch.is_alphabetic() || self.ch == '_' {
            self.read_char()
        }

        let end_index = self.position;

        self.input[start_index..end_index].iter().collect()
    }

    fn read_number(&mut self) -> i32 {
        let start_index = self.position;

        while self.ch.is_digit(10) {
            self.read_char();
        }

        let end_index = self.position;

        self.input[start_index..end_index]
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .expect("Error in parsing sequence of numbers")
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }
}
