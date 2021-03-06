#[cfg(test)]
mod evaluator_test {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        evaluator::{environment::Env, eval},
        parser::parser_test::parse,
    };

    fn test_runner(test_case: &[(&str, &str)]) {
        let env: Env = Rc::new(RefCell::new(Default::default()));

        for (input, expected) in test_case {
            match parse(input) {
                Ok(node) => match eval(node, &Rc::clone(&env)) {
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
            ("true && true", "true"),
            ("true && false", "false"),
            ("false && false", "false"),
            ("true || true", "true"),
            ("false || true", "true"),
            ("false || false", "false"),
            ("true && 0", "false"),
            ("true && 1", "true"),
            ("true || 0", "true"),
            ("true || 1", "true"),
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

    #[test]
    fn test_if_else_expressions() {
        let tests = [
            ("if (true) { 10 }", "10"),
            ("if (false) { 10 }", "null"),
            ("if (1) { 10 }", "10"),
            ("if (1 < 2) { 10 }", "10"),
            ("if (1 > 2) { 10 }", "null"),
            ("if (1 > 2) { 10 } else { 20 }", "20"),
            ("if (1 < 2) { 10 } else { 20 }", "10"),
        ];
        test_runner(&tests);
    }

    #[test]
    fn test_while_expressions() {
        let tests = [("let i = 0; while (i < 10) { i = i + 1;} i;", "10")];
        test_runner(&tests);
    }

    #[test]
    fn test_return_early_in_loop() {
        let tests = [(
            r#"
            let i = 0;
            while(i != 5) {
              if(i == 2) {
                return i;
              }
              i = i + 1;
            }
            i;
            "#,
            "2",
        )];
        test_runner(&tests);
    }

    #[test]
    fn test_return_statements() {
        let tests = [
            ("return 10;", "10"),
            ("return 10; 9;", "10"),
            ("return 2 * 5; 9;", "10"),
            ("9; return 2 * 5; 9;", "10"),
            ("if (10 > 1) { return 10; }", "10"),
            (
                "if (10 > 1) {
                  if (10 > 1) {
                   return 10;
                  }
                  return 1;
                }",
                "10",
            ),
        ];
        test_runner(&tests);
    }

    #[test]
    fn test_let_statements() {
        let tests = [
            ("let a = 5; a;", "5"),
            ("let a = 5 * 5; a;", "25"),
            ("let a = 5; let b = a; b;", "5"),
            ("let a = 5; let b = a; let c = a + b + 5; c;", "15"),
            ("let a = 5; a = a + 2; a;", "7"),
        ];
        test_runner(&tests);
    }

    #[test]
    fn test_function_object() {
        let tests = [("fn(x) { x + 2; };", "fn(x) {...}")];
        test_runner(&tests);
    }

    #[test]
    fn test_function_application() {
        let tests = [
            ("let identity = fn(x) { x; }; identity(5);", "5"),
            ("let identity = fn(x) { return x; }; identity(5);", "5"),
            ("let double = fn(x) { x * 2; }; double(5);", "10"),
            ("let add = fn(x, y) { x + y; }; add(5, 5);", "10"),
            ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", "20"),
            ("fn(x) { x; }(5)", "5"),
        ];
        test_runner(&tests);
    }

    #[test]
    fn test_closure() {
        let tests = [(
            "let newAdder = fn(x) {
             fn(y) { x + y };
             };
             let addTwo = newAdder(2);
             addTwo(2);",
            "4",
        )];
        test_runner(&tests);
    }

    #[test]
    fn test_string_expression() {
        let test_case = [(r#""Hello there!""#, "Hello there!")];
        test_runner(&test_case);
    }

    #[test]
    fn test_string_concatenation() {
        let test_case = [(r#""Hello" + " " + "there!""#, "Hello there!")];
        test_runner(&test_case);
    }

    #[test]
    fn test_builtin_functions() {
        let test_case = [
            (r#"len("")"#, "0"),
            (r#"len("four")"#, "4"),
            (r#"len("hello world")"#, "11"),
            ("len(1)", "Argument to `len` not supported, got 1"),
            (r#"len("one", "two")"#, "Invalid number of arguments: expected=1, got=2"),
            ("push([], 1)", "[1]"),
        ];
        test_runner(&test_case);
    }

    #[test]
    fn test_array_literals() {
        let test_case = [
            (r#"[]"#, "[]"),
            (r#"[1, 2 * 2, 3 + 3]"#, "[1, 4, 6]"),
            (r#"[1 + 1, 2 * 2, 3 + 3]"#, "[2, 4, 6]"),
        ];
        test_runner(&test_case);
    }

    #[test]
    fn test_index_expression() {
        let test_case = [
            ("[1, 2, 3][0]", "1"),
            ("[1, 2, 3][1]", "2"),
            ("[1, 2, 3][2]", "3"),
            ("let i = 0; [1][i];", "1"),
            ("[1, 2, 3][1 + 1];", "3"),
            ("let myArray = [1, 2, 3]; myArray[2];", "3"),
            ("let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];", "6"),
            ("let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]", "2"),
            ("[1, 2, 3][3]", "null"),
            ("[1, 2, 3][-1]", "null"),
        ];
        test_runner(&test_case);
    }
}
