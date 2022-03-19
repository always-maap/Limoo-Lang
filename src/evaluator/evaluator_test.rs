use crate::parser::parser_test::parse;

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
mod evaluator_test {
    use super::*;

    #[test]
    fn test_integer_expression() {
        let tests = [("5", "5"), ("10", "10")];
        test_runner(&tests);
    }
}
