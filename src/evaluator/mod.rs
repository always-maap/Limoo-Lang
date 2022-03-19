use std::rc::Rc;

use crate::{
    ast::{Expression, Literal, Node},
    object::Object,
};

use self::error::EvaluatorError;

mod error;
mod evaluator_test;

pub type EvaluatorResult = Result<Rc<Object>, EvaluatorError>;

pub fn eval(node: Node) -> EvaluatorResult {
    match node {
        Node::Expr(expression) => eval_expression(&expression),
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
        _ => unimplemented!(),
    }
}
