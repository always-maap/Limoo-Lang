use std::io::{self, Write};

use crate::parser::parser_test::parse;

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

        println!("{}", parse(&input).unwrap());
    }
}
