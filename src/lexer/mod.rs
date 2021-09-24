use crate::token::Token;

#[cfg(test)]
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

    fn next_token(&mut self) -> Token {
        let token: Token;

        match self.ch {
            '=' => token = Token::ASSIGN,
            ';' => token = Token::SEMICOLON,
            '(' => token = Token::LPAREN,
            ')' => token = Token::RPAREN,
            ',' => token = Token::COMMA,
            '+' => token = Token::PLUS,
            '{' => token = Token::LBRACE,
            '}' => token = Token::RBRACE,
            _ => token = Token::EOF,
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
}
