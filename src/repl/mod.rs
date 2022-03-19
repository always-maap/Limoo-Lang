use crate::{ast::Node, lexer::Lexer, parser::Parser};
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

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();

        println!("{}", Node::Program(program));
    }
}
