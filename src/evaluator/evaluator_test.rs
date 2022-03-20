use crate::parser::parser_test::parse;

use super::eval;

fn test_runner(test_case: &[(&str, &str)]) {
    for (input, expected) in test_case {
        match parse(input) {
            Ok(node) => match eval(node) {
                Ok(actual) => assert_eq!(expected, &format!("{}", actual)),
                Err(err) => assert_eq!(expected, &format!("{}", err)),
            },
            Err(err) => {
                println!(
                    "for input: {} the expected was: {}, which didn't passed!",
                    input, expected
                );
                panic!("Parsing Error: {:#?}", err)
            }
        }
    }
}

#[cfg(test)]
mod evaluator_test {
    use super::*;

    #[test]
    fn test_integer_expression() {
        let tests = [("5", "5"), ("10", "10"), ("-5", "-5"), ("-10", "-10")];
        test_runner(&tests);
    }

    #[test]
    fn test_boolean_expression() {
        let tests = [("true", "true"), ("false", "false")];
        test_runner(&tests);
    }

    #[test]
    fn test_bang_operator() {
        let tests = [
            ("!true", "false"),
            ("!false", "true"),
            ("!5", "false"),
            ("!!true", "true"),
            ("!!5", "true"),
        ];
        test_runner(&tests);
    }
}
