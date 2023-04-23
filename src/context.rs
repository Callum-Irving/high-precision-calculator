use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::ast::{BuiltinFunc, CalcFunc};
use crate::{CalcError, CalcResult, Number, PREC, RM};

lazy_static! {
    pub static ref BUILTINS: HashMap<String, CalcFunc> = {
        let mut m = HashMap::new();

        // Square root function
        m.insert(
            "sqrt".to_string(),
            CalcFunc::Builtin(BuiltinFunc::new(1, |args, _ctx| {
                Ok(args[0].clone().sqrt(PREC, RM))
            })),
        );

        // Trig functions
        m.insert(
            "sin".to_string(),
            CalcFunc::Builtin(BuiltinFunc::new(1, |args, _ctx| {
                let mut consts = astro_float::Consts::new().unwrap();
                Ok(args[0].clone().sin(PREC, RM, &mut consts))
            }))
        );

        m.insert(
            "cos".to_string(),
            CalcFunc::Builtin(BuiltinFunc::new(1, |args, _ctx| {
                let mut consts = astro_float::Consts::new().unwrap();
                Ok(args[0].clone().cos(PREC, RM, &mut consts))
            }))
        );

        m.insert(
            "tan".to_string(),
            CalcFunc::Builtin(BuiltinFunc::new(1, |args, _ctx| {
               let mut consts = astro_float::Consts::new().unwrap();
                Ok(args[0].clone().tan(PREC, RM, &mut consts))
            }))
        );

        m
    };
}

#[derive(Clone)]
pub struct Context {
    functions: Vec<HashMap<String, CalcFunc>>,
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
            .ok_or(CalcError::NameNotFound(name.to_owned()))
    }

    pub fn lookup_fn(&self, name: &str) -> Result<&CalcFunc, CalcError> {
        if let Some(func) = self
            .functions
            .iter()
            .rev()
            .find(|s| s.contains_key(name))
            .and_then(|s| s.get(name))
        {
            Ok(func)
        } else if let Some(func) = BUILTINS.get(name) {
            Ok(func)
        } else {
            Err(CalcError::NameNotFound(name.to_owned()))
        }
    }

    pub fn bind_value(&mut self, name: String, value: Number) -> CalcResult {
        self.values.last_mut().unwrap().insert(name, value.clone());
        Ok(value)
    }

    pub fn bind_fn(&mut self, name: String, func: CalcFunc) -> Result<(), CalcError> {
        // Make sure you don't overwrite a builtin
        if BUILTINS.contains_key(&name) {
            Err(CalcError::NameAlreadyBound(name))
        } else {
            self.functions.last_mut().unwrap().insert(name, func);
            Ok(())
        }
    }
}
