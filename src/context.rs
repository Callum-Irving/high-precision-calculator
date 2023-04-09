use std::collections::HashMap;

use crate::ast::Callable;
use crate::{CalcError, CalcResult, Number};

#[derive(Clone)]
pub struct Context {
    functions: Vec<HashMap<String, Box<dyn Callable>>>,
    values: Vec<HashMap<String, Number>>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            functions: vec![HashMap::new()],
            values: vec![HashMap::new()],
        }
    }

    pub fn add_scope(&mut self, values: HashMap<String, Number>) {
        self.values.push(values);
    }

    pub fn lookup_value(&self, name: &str) -> CalcResult {
        self.values
            .iter()
            .rev()
            .find(|map| map.contains_key(name))
            .and_then(|map| map.get(name).cloned())
            .ok_or(CalcError::NameNotFound)
    }

    pub fn lookup_fn(&self, name: &str) -> Result<&Box<dyn Callable>, CalcError> {
        self.functions
            .iter()
            .rev()
            .find(|map| map.contains_key(name))
            .and_then(|map| map.get(name))
            .ok_or(CalcError::NameNotFound)
    }

    pub fn bind_value(&mut self, name: String, value: Number) -> CalcResult {
        // TODO: Only bind if not already exists
        self.values
            .last_mut()
            .expect("empty context")
            .insert(name, value.clone())
            .map_or(Ok(value), |_| Err(CalcError::NameAlreadyBound))
    }

    pub fn bind_fn(&mut self, name: String, func: Box<dyn Callable>) -> Result<(), CalcError> {
        // TODO: Only bind if not already exists
        self.functions
            .last_mut()
            .expect("empty context")
            .insert(name, func)
            .map_or(Ok(()), |_| Err(CalcError::NameAlreadyBound))
    }
}
