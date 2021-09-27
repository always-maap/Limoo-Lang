use crate::{lexer::Lexer, token::Token};
use std::io::{self, Write};

const PROMPT: &str = ">> ";

pub fn start() {
    println!("Limoo 🍋  v0.0.1 repl!");

    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mut lexer = Lexer::new(&input);

        loop {
            let token = lexer.next_token();

            println!("{:?}", &token);

            if token == Token::EOF {
                break;
            }
        }
    }
}
