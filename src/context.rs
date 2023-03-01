use crate::function::InnerFunction;
use crate::value::Value;
use core::clone::Clone;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub enum ContextValue {
    Variable(Value),
    Function(Arc<InnerFunction>),
}

pub struct Context(pub Arc<Mutex<HashMap<String, ContextValue>>>);

impl Context {
    pub fn new() -> Self {
        Context(Arc::new(Mutex::new(HashMap::new())))
    }

    pub fn set_func(&mut self, name: &String, func: Arc<InnerFunction>) {
        self.0
            .lock()
            .unwrap()
            .insert(name.clone(), ContextValue::Function(func.clone()));
    }

    pub fn set_variable(&mut self, name: &str, value: Value) {
        self.0
            .lock()
            .unwrap()
            .insert(name.to_string(), ContextValue::Variable(value));
    }

    pub fn get_func(&self, name: &str) -> Option<Arc<InnerFunction>> {
        let binding = self.0.lock().unwrap();
        let value = binding.get(name)?;
        match value {
            ContextValue::Function(func) => Some(func.clone()),
            ContextValue::Variable(_) => None,
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<Value> {
        let binding = self.0.lock().unwrap();
        let value = binding.get(name)?;
        match value {
            ContextValue::Variable(v) => Some(v.clone()),
            ContextValue::Function(_) => None,
        }
    }

    pub fn get(&self, name: &String) -> Option<ContextValue> {
        let binding = self.0.lock().unwrap();
        let value = binding.get(name)?;
        Some(value.clone())
    }
}
