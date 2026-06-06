use std::collections::HashMap;

use crate::{
    interpreter::{LoxValue, RuntimeError},
    token::Token,
};

pub(crate) struct Environment {
    variables: HashMap<String, LoxValue>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub(crate) fn define(&mut self, name: String, value: LoxValue) {
        self.variables.insert(name, value);
    }

    pub(crate) fn get(&self, name: Token) -> Result<&LoxValue, RuntimeError> {
        let value = self.variables.get(&name.lexeme);
        value.ok_or(RuntimeError {
            message: format!("Undefined variable '{}'.", name.lexeme),
        })
    }
}
