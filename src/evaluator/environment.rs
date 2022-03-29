use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::Object;

pub type Env = Rc<RefCell<Environment>>;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Environment {
    store: HashMap<String, Rc<Object>>,
    outer: Option<Env>,
}

impl Environment {
    pub fn new_enclosed_environment(outer: &Env) -> Self {
        let mut env: Environment = Environment::default();
        env.outer = Some(Rc::clone(outer));
        env
    }
    pub fn get(&self, name: &str) -> Option<Rc<Object>> {
        match self.store.get(name) {
            Some(object) => Some(Rc::clone(object)),
            None => {
                if let Some(outer) = &self.outer {
                    outer.borrow().get(name)
                } else {
                    None
                }
            }
        }
    }

    pub fn set(&mut self, name: String, object: Rc<Object>) {
        self.store.insert(name, object);
    }
}
