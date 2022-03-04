use crate::{ast::Node, lexer::Lexer, parser::Parser};

use super::error::ParserErrors;

pub fn parse(input: &str) -> Result<Node, ParserErrors> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;

    Ok(Node::Program(program))
}

fn test_runner(test_case: &[(&str, &str)]) {
    for (input, expected) in test_case {
        match parse(input) {
            Ok(node) => assert_eq!(expected, &format!("{}", node)),
            Err(e) => panic!("Parsing Error: {:#?}", e),
        }
    }
}

#[test]
fn test_let_statements() {
    let tests = [
        ("let x = 5;", "let x = 5;"),
        ("let foobar = y;", "let foobar = y;"),
    ];

    test_runner(&tests);
}

#[test]
fn test_return_statements() {
    let tests = [
        ("return 5;", "return 5;"),
        ("return foobar;", "return foobar;"),
    ];

    test_runner(&tests);
}

#[test]
fn test_identifier_expression() {
    let tests = [("foobar;", "foobar")];

    test_runner(&tests);
}

#[test]
fn test_integer_expression() {
    let tests = [("5;", "5")];

    test_runner(&tests);
}
