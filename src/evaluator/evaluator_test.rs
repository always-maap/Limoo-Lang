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
        let tests = [
            ("5", "5"),
            ("10", "10"),
            ("-5", "-5"),
            ("-10", "-10"),
            ("5 + 5 + 5 + 5 - 10", "10"),
            ("2 * 2 * 2 * 2 * 2", "32"),
            ("-50 + 100 + -50", "0"),
            ("5 * 2 + 10", "20"),
            ("5 + 2 * 10", "25"),
            ("20 + 2 * -10", "0"),
            ("50 / 2 * 2 + 10", "60"),
            ("2 * (5 + 10)", "30"),
            ("3 * 3 * 3 + 10", "37"),
            ("3 * (3 * 3) + 10", "37"),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", "50"),
        ];
        test_runner(&tests);
    }

    #[test]
    fn test_boolean_expression() {
        let tests = [
            ("true", "true"),
            ("false", "false"),
            ("1 < 2", "true"),
            ("1 > 2", "false"),
            ("1 < 1", "false"),
            ("1 > 1", "false"),
            ("1 == 1", "true"),
            ("1 != 1", "false"),
            ("1 == 2", "false"),
            ("1 != 2", "true"),
            ("true == true", "true"),
            ("false == false", "true"),
            ("true == false", "false"),
            ("true != false", "true"),
            ("false != true", "true"),
            ("(1 < 2) == true", "true"),
            ("(1 < 2) == false", "false"),
            ("(1 > 2) == true", "false"),
            ("(1 > 2) == false", "true"),
        ];
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
