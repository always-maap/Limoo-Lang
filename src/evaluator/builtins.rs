use core::fmt;
use std::rc::Rc;

use crate::object::Object;

use super::error::EvaluatorError;

#[derive(Debug, Clone, PartialEq)]
pub enum Builtin {
    Len,
    Print,
    Push,
}

impl Builtin {
    pub fn lookup(name: &str) -> Option<Object> {
        match name {
            "len" => Some(Object::Builtin(Builtin::Len)),
            "print" => Some(Object::Builtin(Builtin::Print)),
            "push" => Some(Object::Builtin(Builtin::Push)),
            _ => None,
        }
    }

    pub fn apply(&self, args: &[Rc<Object>]) -> Result<Rc<Object>, EvaluatorError> {
        match self {
            Builtin::Len => {
                check_argument_count(1, args.len())?;

                match &*args[0] {
                    Object::String(string) => Ok(Rc::new(Object::Integer(string.len() as i32))),
                    Object::Array(array) => Ok(Rc::new(Object::Integer(array.len() as i32))),
                    object => Err(EvaluatorError::new(format!(
                        "Argument to `len` not supported, got {}",
                        object
                    ))),
                }
            }
            Builtin::Print => {
                args.iter().for_each(|obj| println!("{}", obj));
                Ok(Rc::new(Object::Null))
            }
            Builtin::Push => {
                check_argument_count(2, args.len())?;

                let array = args.first().unwrap();
                let object = Rc::clone(args.last().unwrap());

                match &**array {
                    Object::Array(arr) => {
                        let mut new_arr = arr.clone();
                        new_arr.push(object);
                        Ok(Rc::new(Object::Array(new_arr)))
                    }
                    object => Err(EvaluatorError::new(format!(
                        "Argument to `push` not supported, got {}",
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
            Builtin::Print => write!(f, "print"),
            Builtin::Push => write!(f, "push"),
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
