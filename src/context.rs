use std::collections::HashMap;

use crate::ast::Callable;
use crate::{CalcError, CalcResult, Number, PREC, RM};

// Builtin functions
#[derive(Clone)]
struct SqrtFn;

impl Callable for SqrtFn {
    fn arity(&self) -> usize {
        1
    }

    fn call(&self, args: &[Number], _ctx: &Context) -> CalcResult {
        Ok(args[0].clone().sqrt(PREC, RM))
    }
}

fn builtins() -> HashMap<String, Box<dyn Callable>> {
    let map: HashMap<String, Box<dyn Callable>> =
        HashMap::from([("sqrt".to_string(), Box::new(SqrtFn) as Box<dyn Callable>)]);
    map
}

#[derive(Clone)]
pub struct Context {
    functions: Vec<HashMap<String, Box<dyn Callable>>>,
    values: Vec<HashMap<String, Number>>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            functions: vec![builtins()],
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
        if self
            .values
            .last()
            .expect("empty context")
            .contains_key(&name)
        {
            Err(CalcError::NameAlreadyBound)
        } else {
            self.values.last_mut().unwrap().insert(name, value.clone());
            Ok(value)
        }
    }

    pub fn bind_fn(&mut self, name: String, func: Box<dyn Callable>) -> Result<(), CalcError> {
        if self
            .functions
            .last()
            .expect("empty context")
            .contains_key(&name)
        {
            Err(CalcError::NameAlreadyBound)
        } else {
            self.functions.last_mut().unwrap().insert(name, func);
            Ok(())
        }
    }
}
