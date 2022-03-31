use core::fmt;
use std::rc::Rc;

use crate::object::Object;

use super::error::EvaluatorError;

#[derive(Debug, Clone, PartialEq)]
pub enum Builtin {
    Len,
}

impl Builtin {
    pub fn lookup(name: &str) -> Option<Object> {
        match name {
            "len" => Some(Object::Builtin(Builtin::Len)),
            _ => None,
        }
    }

    pub fn apply(&self, args: &[Rc<Object>]) -> Result<Rc<Object>, EvaluatorError> {
        match self {
            Builtin::Len => {
                check_argument_count(1, args.len())?;

                match &*args[0] {
                    Object::String(string) => Ok(Rc::new(Object::Integer(string.len() as i32))),
                    object => Err(EvaluatorError::new(format!(
                        "Argument to `len` not supported, got {}",
                        object
                    ))),
                }
            }
        }
    }
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Builtin::Len => write!(f, "len"),
        }
    }
}

fn check_argument_count(expected: usize, actual: usize) -> Result<(), EvaluatorError> {
    if expected != actual {
        Err(EvaluatorError::new(format!(
            "Invalid number of arguments: expected={}, got={}",
            expected, actual
        )))
    } else {
        Ok(())
    }
}
