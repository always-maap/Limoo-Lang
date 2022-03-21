use std::{
    cell::RefCell,
    io::{self, Write},
    rc::Rc,
};

use crate::{
    evaluator::{environment::Env, eval},
    parser::parser_test::parse,
};

const PROMPT: &str = ">> ";

pub fn start() {
    let env: Env = Rc::new(RefCell::new(Default::default()));

    println!("Limoo 🍋  v0.0.1 repl!");

    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        match parse(&input) {
            Ok(node) => match eval(node, &Rc::clone(&env)) {
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
