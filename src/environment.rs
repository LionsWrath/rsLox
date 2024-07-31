use std::collections::HashMap;
use crate::ast_expr::*;
use crate::error::RuntimeError;

pub struct Environment {
    values: HashMap<String, Literal>,
    enclosing: Option<&mut Environment>,
}

impl Environment {
    pub fn new() -> Self {

        let values = HashMap::new();

        Environment {
            values,
            None
        }
    }

    pub fn new_enclosing(enclosing: &mut Environment) -> Self {

        let values = HashMap::new();

        Environment {
            values,
            enclosing
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &String) -> &Literal {
        if self.values.contains_key(name) {
            return &self.values[name];
        }

        match self.enclosing {
            Some(env) => return env.get(name),
            None => panic!("Undefined variable {}", name)
        }
    }

    pub fn assign(&mut self, name: String, value: Literal) {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return;
        }

        match self.enclosing {
            Some(env) => env.get(name),
            None => panic!("Undefined variable {}", name)
        }
    }
}
