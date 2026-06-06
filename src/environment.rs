use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    interpreter::{LoxValue, RuntimeError},
    token::Token,
};

pub(crate) struct Environment {
    variables: HashMap<String, LoxValue>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Self {
            variables: HashMap::new(),
            parent: None,
        }
    }

    pub(crate) fn new_with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            variables: HashMap::new(),
            parent: Some(parent),
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
            Some(parent) => parent.borrow_mut().assign(name, value),
            None => Err(RuntimeError {
                message: format!("Undefined variable '{}'.", name.lexeme),
            }),
        }
    }

    pub(crate) fn get(&self, name: Token) -> Result<LoxValue, RuntimeError> {
        let value = self.variables.get(&name.lexeme);
        match value {
            Some(val) => Ok(val.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().get(name),
                None => Err(RuntimeError {
                    message: format!("Undefined variable '{}'.", name.lexeme),
                }),
            },
        }
    }
}
