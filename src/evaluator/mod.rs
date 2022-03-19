use std::rc::Rc;

use crate::{
    ast::{Expression, Literal, Node, Statement},
    object::Object,
};

use self::error::EvaluatorError;

mod error;
mod evaluator_test;

pub type EvaluatorResult = Result<Rc<Object>, EvaluatorError>;

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
