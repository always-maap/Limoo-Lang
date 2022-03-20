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

pub fn eval(node: Node) -> EvaluatorResult {
    //println!("{:#?}", node);
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
