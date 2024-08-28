use std::collections::HashMap;
use crate::ast_expr::*;

pub struct Environment {
    values: HashMap<String, Literal>,
    enclosing: Option<Box<Self>>,
}

impl Environment {
    pub fn new() -> Self {

        let values = HashMap::new();
        let enclosing = None;

        Environment {
            values,
            enclosing
        }
    }

    pub fn add_enclosing(env: & mut Environment) {

        let values = HashMap::new();
        let enclosing = None;

        let new_enclosing = Some(
            Box::new(
                Environment {
                    values,
                    enclosing
                }
            )
        );

        env.enclosing = new_enclosing;
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &String) -> &Literal {
        if self.values.contains_key(name) {
            return &self.values[name];
        }

        match &mut self.enclosing {
            Some(env) => return env.get(name),
            None => panic!("Undefined variable {}", name)
        }
    }

    // Review this function
    pub fn assign(&mut self, name: String, value: Literal) {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return;
        }

        match &mut self.enclosing {
            Some(env) => env.assign(name, value),
            None => panic!("Undefined variable {}", name)
        }
    }
}
