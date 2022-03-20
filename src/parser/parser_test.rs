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
            Err(e) => {
                println!(
                    "for input: {} the expected was: {}, which didn't passed!",
                    input, expected
                );
                panic!("Parsing Error: {:#?}", e)
            }
        }
    }
}

#[cfg(test)]
pub mod parser_test {
    use super::*;

    #[test]
    fn test_let_statements() {
        let tests = [
            ("let x = 5;", "let x = 5;"),
            ("let y = true;", "let y = true;"),
            ("let foobar = y;", "let foobar = y;"),
        ];

        test_runner(&tests);
    }

    #[test]
    fn test_return_statements() {
        let tests = [
            ("return 5;", "return 5;"),
            ("return true;", "return true;"),
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
    fn test_integer_literal_expression() {
        let tests = [("5;", "5")];

        test_runner(&tests);
    }

    #[test]
    fn test_parse_prefix_expression() {
        let tests = [
            ("!5;", "(!5)"),
            ("-15;", "(-15)"),
            ("!foobar;", "(!foobar)"),
            ("-foobar;", "(-foobar)"),
            ("!true;", "(!true)"),
            ("!false;", "(!false)"),
        ];

        test_runner(&tests);
    }

    #[test]
    fn test_parse_infix_expression() {
        let tests = [
            ("5 + 5;", "(5 + 5)"),
            ("5 - 5;", "(5 - 5)"),
            ("5 * 5;", "(5 * 5)"),
            ("5 / 5;", "(5 / 5)"),
            ("5 > 5;", "(5 > 5)"),
            ("5 < 5;", "(5 < 5)"),
            ("5 == 5;", "(5 == 5)"),
            ("5 != 5;", "(5 != 5)"),
        ];

        test_runner(&tests);
    }

    #[test]
    fn test_operator_precedence() {
        let tests = [
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            ("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("(5 + 5) * 2 * (5 + 5)", "(((5 + 5) * 2) * (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
        ];

        test_runner(&tests);
    }

    #[test]
    fn test_if_expression() {
        let test_case = [("if (x < y) { x }", "if (x < y) { x }")];
        test_runner(&test_case);
    }

    #[test]
    fn test_if_else_expression() {
        let test_case = [("if (x < y) { x } else { y }", "if (x < y) { x } else { y }")];
        test_runner(&test_case);
    }

    #[test]
    fn test_function_expression() {
        let test_case = [
            ("fn() {};", "fn() {...}"),
            ("fn(x) {};", "fn(x) {...}"),
            ("fn(x, y, z) {};", "fn(x, y, z) {...}"),
        ];
        test_runner(&test_case);
    }

    #[test]
    fn test_fn_call_expression() {
        let test_case = [("add(1, 2 * 3, 4 + 5);", "add(1, (2 * 3), (4 + 5))")];
        test_runner(&test_case);
    }
}
