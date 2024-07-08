use std::collections::HashMap;
use crate::ast_expr::*;

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
}
