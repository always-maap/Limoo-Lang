use std::io::{self, Write};

use crate::{evaluator::eval, parser::parser_test::parse};

const PROMPT: &str = ">> ";

pub fn start() {
    println!("Limoo 🍋  v0.0.1 repl!");

    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        match parse(&input) {
            Ok(node) => match eval(node) {
                Ok(value) => println!("{}", value),
                Err(err) => eprintln!("{}", err),
            },
            Err(errors) => {
                for error in errors {
                    eprintln!("{}", error);
                }
            }
        }
    }
}
