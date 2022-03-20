use std::rc::Rc;

use crate::{
    ast::{Expression, Literal, Node, Statement},
    object::Object,
    token::Token,
};

use self::error::EvaluatorError;

mod error;
mod evaluator_test;

pub type EvaluatorResult = Result<Rc<Object>, EvaluatorError>;

fn is_truthy(obj: &Object) -> bool {
    match *obj {
        Object::Null => false,
        Object::Boolean(false) => false,
        _ => true,
    }
}

pub fn eval(node: Node) -> EvaluatorResult {
    match node {
        Node::Program(program) => eval_program(&program),
        Node::Expr(expression) => eval_expression(&expression),
        _ => unimplemented!(),
    }
}

fn eval_program(program: &[Statement]) -> EvaluatorResult {
    let result = Rc::new(Object::Null);

    for statement in program {
        let val = eval_statement(statement)?;

        return Ok(val);
    }

    Ok(result)
}

fn eval_statement(statement: &Statement) -> EvaluatorResult {
    match statement {
        Statement::Expr(expression) => eval_expression(expression),
        _ => unimplemented!(),
    }
}

fn eval_expression(expression: &Expression) -> EvaluatorResult {
    match expression {
        Expression::Lit(c) => eval_literal(c),
        Expression::Prefix(operator, expression) => {
            let right = eval_expression(expression)?;
            eval_prefix_expression(operator, &right)
        }
        Expression::Infix(left, operator, right) => {
            let left = eval_expression(left)?;
            let right = eval_expression(right)?;
            eval_infix_expression(&left, operator, &right)
        }
        Expression::If(condition, consequence, alternative) => {
            let condition = eval_expression(condition)?;

            if is_truthy(&condition) {
                eval_block_statement(consequence)
            } else {
                match alternative {
                    Some(alternative) => eval_block_statement(alternative),
                    None => Ok(Rc::new(Object::Null)),
                }
            }
        }
        _ => unimplemented!(),
    }
}

fn eval_literal(literal: &Literal) -> EvaluatorResult {
    match literal {
        Literal::Integer(i) => Ok(Rc::new(Object::Integer(*i))),
        Literal::Boolean(b) => Ok(Rc::new(Object::Boolean(*b))),
        _ => unimplemented!(),
    }
}

fn eval_prefix_expression(operator: &Token, right: &Rc<Object>) -> EvaluatorResult {
    match operator {
        Token::BANG => eval_bang_operator(right),
        Token::MINUS => eval_minus_operator(right),
        _ => unimplemented!(),
    }
}

fn eval_bang_operator(expression: &Rc<Object>) -> EvaluatorResult {
    match **expression {
        Object::Boolean(b) => Ok(Rc::new(Object::Boolean(!b))),
        Object::Null => Ok(Rc::new(Object::Boolean(true))),
        _ => Ok(Rc::new(Object::Boolean(false))),
    }
}

fn eval_minus_operator(expression: &Rc<Object>) -> EvaluatorResult {
    match **expression {
        Object::Integer(i) => Ok(Rc::new(Object::Integer(-i))),
        _ => Err(EvaluatorError::new(format!(
            "Unknown operator: -{}",
            expression
        ))),
    }
}

fn eval_infix_expression(
    left: &Rc<Object>,
    operator: &Token,
    right: &Rc<Object>,
) -> EvaluatorResult {
    match (&**left, &**right) {
        (Object::Integer(left), Object::Integer(right)) => {
            eval_integer_infix_expression(*left, operator, *right)
        }
        (Object::Boolean(left), Object::Boolean(right)) => {
            eval_boolean_infix_expression(*left, operator, *right)
        }
        _ => Err(EvaluatorError::new(format!(
            "Mismatch type: {} {} {}",
            left, operator, right
        ))),
    }
}

fn eval_integer_infix_expression(left: i32, operator: &Token, right: i32) -> EvaluatorResult {
    let result = match operator {
        Token::PLUS => Object::Integer(left + right),
        Token::MINUS => Object::Integer(left - right),
        Token::ASTERISK => Object::Integer(left * right),
        Token::SLASH => Object::Integer(left / right),
        Token::EQ => Object::Boolean(left == right),
        Token::NOT_EQ => Object::Boolean(left != right),
        Token::LT => Object::Boolean(left < right),
        Token::GT => Object::Boolean(left > right),
        _ => {
            return Err(EvaluatorError::new(format!(
                "Unknown operator: {} {} {}",
                left, operator, right
            )))
        }
    };

    Ok(Rc::new(result))
}

fn eval_boolean_infix_expression(left: bool, operator: &Token, right: bool) -> EvaluatorResult {
    let result = match operator {
        Token::EQ => Object::Boolean(left == right),
        Token::NOT_EQ => Object::Boolean(left != right),
        _ => {
            return Err(EvaluatorError::new(format!(
                "Unknown operator: {} {} {}",
                left, operator, right
            )))
        }
    };

    Ok(Rc::new(result))
}

fn eval_block_statement(statements: &[Statement]) -> EvaluatorResult {
    let result = Rc::new(Object::Null);

    for statement in statements {
        let val = eval_statement(statement)?;

        return Ok(val);
    }

    Ok(result)
}
