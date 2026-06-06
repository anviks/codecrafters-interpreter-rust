use std::collections::HashMap;

use crate::{
    interpreter::{LoxValue, RuntimeError},
    token::Token,
};

pub(crate) struct Environment {
    variables: HashMap<String, LoxValue>,
    pub(crate) parent: Option<Box<Environment>>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Self {
            variables: HashMap::new(),
            parent: None,
        }
    }

    pub(crate) fn define(&mut self, name: String, value: LoxValue) {
        self.variables.insert(name, value);
    }

    pub(crate) fn assign(&mut self, name: Token, value: LoxValue) -> Result<(), RuntimeError> {
        if self.variables.contains_key(&name.lexeme) {
            self.variables.insert(name.lexeme.clone(), value);
            return Ok(());
        }

        match &mut self.parent {
            Some(parent) => parent.assign(name, value),
            None => Err(RuntimeError {
                message: format!("Undefined variable '{}'.", name.lexeme),
            }),
        }
    }

    pub(crate) fn get(&self, name: Token) -> Result<&LoxValue, RuntimeError> {
        let value = self.variables.get(&name.lexeme);
        match value {
            Some(val) => Ok(val),
            None => match &self.parent {
                Some(parent) => parent.get(name),
                None => Err(RuntimeError {
                    message: format!("Undefined variable '{}'.", name.lexeme),
                }),
            },
        }
    }
}
