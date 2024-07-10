use std::collections::HashMap;
use crate::ast_expr::*;
use crate::error::RuntimeError;

pub struct Environment {
    values: HashMap<String, Literal>
}

impl Environment {
    pub fn new() -> Self {

        let values = HashMap::new();

        Environment {
            values
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &String) -> Result<&Literal, RuntimeError> {
        if self.values.contains_key(name) {
            return Ok(&self.values[name]);
        }

        Err(RuntimeError::new(format!("Undefined variable {}", name.to_string())))
    }
}
