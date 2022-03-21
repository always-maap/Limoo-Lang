use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::Object;

pub type Env = Rc<RefCell<Environment>>;

#[derive(Debug, Clone, Default)]
pub struct Environment {
    store: HashMap<String, Rc<Object>>,
}

impl Environment {
    pub fn get(&self, name: &str) -> Option<Rc<Object>> {
        match self.store.get(name) {
            Some(object) => Some(Rc::clone(object)),
            None => None,
        }
    }

    pub fn set(&mut self, name: String, object: Rc<Object>) {
        self.store.insert(name, object);
    }
}
