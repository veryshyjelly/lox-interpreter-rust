use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use super::*;

#[derive(Default, Clone)]
pub struct Env {
    pub values: HashMap<String, Object>,
    pub next: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new(next: Option<Rc<RefCell<Env>>>) -> Self {
        Self {
            values: HashMap::new(),
            next,
        }
    }

    pub fn new_box_it(next: Option<Rc<RefCell<Env>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(next)))
    }
}

pub fn find_id<'a>(id: &String, env: Option<Rc<RefCell<Env>>>) -> Option<Rc<RefCell<Env>>> {
    let mut current_env = env;
    while let Some(env) = current_env {
        if env.borrow().values.contains_key(id) {
            return Some(env);
        }
        current_env = env.borrow().next.clone();
    }
    None
}

impl Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in &self.values {
            writeln!(f, "key: {}, value: {}", k, v)?;
        }

        if let Some(v) = &self.next {
            v.borrow().fmt(f)?;
        }

        Ok(())
    }
}
