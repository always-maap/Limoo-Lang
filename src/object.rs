use core::fmt;
use std::rc::Rc;

use crate::{
    ast::BlockStatement,
    evaluator::{builtins::Builtin, environment::Env},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    String(String),
    Null,
    ReturnValue(Rc<Object>),
    Function(Vec<String>, BlockStatement, Env),
    Builtin(Builtin),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::String(s) => write!(f, "{}", s),
            Object::Null => write!(f, "null"),
            Object::ReturnValue(obj) => write!(f, "{}", obj),
            Object::Function(params, _body, _env) => {
                write!(f, "fn({}) {{...}}", params.join(","))
            }
            Object::Builtin(builtin) => write!(f, "Builtin Function: {}", builtin),
        }
    }
}
