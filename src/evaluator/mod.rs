use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{Expression, Literal, Node, Statement},
    object::Object,
    token::Token,
};

use self::{
    builtins::Builtin,
    environment::{Env, Environment},
    error::EvaluatorError,
};

pub mod builtins;
pub mod environment;
mod error;
mod evaluator_test;

pub type EvaluatorResult = Result<Rc<Object>, EvaluatorError>;

fn is_truthy(obj: &Object) -> bool {
    match *obj {
        Object::Null => false,
        Object::Boolean(false) => false,
        Object::String(ref s) if s.is_empty() => false,
        Object::Integer(0) => false,
        _ => true,
    }
}

pub fn eval(node: Node, env: &Env) -> EvaluatorResult {
    match node {
        Node::Program(program) => eval_program(&program, env),
        Node::Stmt(statement) => eval_statement(&statement, env),
        Node::Expr(expression) => eval_expression(&expression, env),
    }
}

fn eval_program(program: &[Statement], env: &Env) -> EvaluatorResult {
    let mut result = Rc::new(Object::Null);

    for statement in program {
        let val = eval_statement(statement, &Rc::clone(env))?;

        match *val {
            Object::ReturnValue(_) => return Ok(val),
            _ => result = val,
        }
    }

    Ok(result)
}

fn eval_statement(statement: &Statement, env: &Env) -> EvaluatorResult {
    match statement {
        Statement::Let(identifier, expression) => {
            let value = eval_expression(expression, &Rc::clone(env))?;
            let object = Rc::clone(&value);
            env.borrow_mut().set(identifier.clone(), object);
            Ok(value)
        }
        Statement::Expr(expression) => eval_expression(expression, env),
        Statement::Return(expression) => {
            let val = eval_expression(expression, env)?;

            return Ok(Rc::new(Object::ReturnValue(val)));
        }
    }
}

fn eval_expression(expression: &Expression, env: &Env) -> EvaluatorResult {
    match expression {
        Expression::Lit(c) => eval_literal(c),
        Expression::Prefix(operator, expression) => {
            let right = eval_expression(expression, env)?;
            eval_prefix_expression(operator, &right)
        }
        Expression::Infix(left, operator, right) => {
            let left = eval_expression(left, &Rc::clone(env))?;
            let right = eval_expression(right, &Rc::clone(env))?;
            eval_infix_expression(&left, operator, &right)
        }
        Expression::If(condition, consequence, alternative) => {
            let condition = eval_expression(condition, &Rc::clone(env))?;

            if is_truthy(&condition) {
                eval_block_statement(consequence, env)
            } else {
                match alternative {
                    Some(alternative) => eval_block_statement(alternative, env),
                    None => Ok(Rc::new(Object::Null)),
                }
            }
        }
        Expression::Ident(identifier) => eval_identifier(identifier, env),
        Expression::Function(params, body) => {
            let function = Rc::new(Object::Function(params.clone(), body.clone(), Rc::clone(env)));
            Ok(function)
        }
        Expression::FunctionCall(function, args) => {
            let func = eval_expression(function, &Rc::clone(env))?;
            let args = eval_expressions(args, env)?;
            apply_function(&func, &args)
        }
        Expression::Assign(identifier, _, expression) => {
            let value = eval_expression(expression, env)?;
            env.borrow_mut().set(identifier.clone(), value.clone());
            Ok(value)
        }
    }
}

fn eval_expressions(expressions: &[Expression], env: &Env) -> Result<Vec<Rc<Object>>, EvaluatorError> {
    let mut list = Vec::new();

    for expr in expressions {
        let val = eval_expression(expr, &Rc::clone(env))?;
        list.push(val);
    }

    Ok(list)
}

fn eval_literal(literal: &Literal) -> EvaluatorResult {
    match literal {
        Literal::Integer(i) => Ok(Rc::new(Object::Integer(*i))),
        Literal::Boolean(b) => Ok(Rc::new(Object::Boolean(*b))),
        Literal::String(s) => Ok(Rc::new(Object::String(s.clone()))),
    }
}

fn eval_prefix_expression(operator: &Token, right: &Rc<Object>) -> EvaluatorResult {
    match operator {
        Token::BANG => eval_bang_operator(right),
        Token::MINUS => eval_minus_operator(right),
        _ => Err(EvaluatorError::new(format!("Unknown operator: {}{}", operator, right))),
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
        _ => Err(EvaluatorError::new(format!("Unknown operator: -{}", expression))),
    }
}

fn eval_infix_expression(left: &Rc<Object>, operator: &Token, right: &Rc<Object>) -> EvaluatorResult {
    if *operator == Token::AND || *operator == Token::OR {
        return eval_logical_infix_expression(left, operator, right);
    }

    match (&**left, &**right) {
        (Object::Integer(left), Object::Integer(right)) => eval_integer_infix_expression(*left, operator, *right),
        (Object::Boolean(left), Object::Boolean(right)) => eval_boolean_infix_expression(*left, operator, *right),
        (Object::String(left), Object::String(right)) => eval_string_infix_expression(left, operator, right),
        _ => Err(EvaluatorError::new(format!(
            "Mismatch type: {} {} {}",
            left, operator, right
        ))),
    }
}

fn eval_logical_infix_expression(left: &Rc<Object>, operator: &Token, right: &Rc<Object>) -> EvaluatorResult {
    let (left_coercion, right_ceorcion) = (is_truthy(left), is_truthy(right));

    let result = match operator {
        Token::AND => left_coercion && right_ceorcion,
        Token::OR => left_coercion || right_ceorcion,
        _ => {
            return Err(EvaluatorError::new(format!(
                "Unknown operator: {} {} {}",
                left, operator, right
            )))
        }
    };

    Ok(Rc::new(Object::Boolean(result)))
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

fn eval_string_infix_expression(left: &str, operator: &Token, right: &str) -> EvaluatorResult {
    let result = match operator {
        Token::PLUS => Object::String(format!("{}{}", left, right)),
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

fn eval_block_statement(statements: &[Statement], env: &Env) -> EvaluatorResult {
    let mut result = Rc::new(Object::Null);

    for statement in statements {
        let val = eval_statement(statement, &Rc::clone(env))?;

        match *val {
            Object::ReturnValue(_) => return Ok(val),
            _ => result = val,
        }
    }

    Ok(result)
}

fn eval_identifier(identifier: &str, env: &Env) -> EvaluatorResult {
    let val = env.borrow().get(identifier);

    match val {
        Some(val) => Ok(val.clone()),
        None => match Builtin::lookup(identifier) {
            Some(object) => Ok(Rc::new(object)),
            None => Err(EvaluatorError::new(format!("Identifier not found: {}", identifier))),
        },
    }
}

fn apply_function(function: &Rc<Object>, args: &[Rc<Object>]) -> EvaluatorResult {
    match &**function {
        Object::Function(params, body, env) => {
            let mut extended_env = Environment::new_enclosed_environment(&Rc::clone(env));

            if params.len() != args.len() {
                return Err(EvaluatorError::new(format!(
                    "Expected {} arguments but got {}",
                    params.len(),
                    args.len()
                )));
            }

            params.iter().enumerate().for_each(|(i, param)| {
                extended_env.set(param.clone(), args[i].clone());
            });

            let evaluted_body = eval_block_statement(&body, &Rc::new(RefCell::new(extended_env)))?;
            unwrap_return_value(evaluted_body)
        }
        Object::Builtin(builtin) => builtin.apply(args),
        _ => Err(EvaluatorError::new(format!("Not a function: {}", function))),
    }
}

fn unwrap_return_value(obj: Rc<Object>) -> EvaluatorResult {
    if let Object::ReturnValue(val) = &*obj {
        Ok(Rc::clone(&val))
    } else {
        Ok(obj)
    }
}
