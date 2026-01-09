use crate::define::Result;
use crate::error::Error;
use crate::value::Value;
use once_cell::sync::OnceCell;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type InnerFunction = dyn Fn(Vec<Value>) -> Result<Value> + Send + Sync + 'static;

pub struct InnerFunctionManager {
    pub store: &'static Mutex<HashMap<String, Arc<InnerFunction>>>,
}

impl InnerFunctionManager {
    pub fn new() -> Self {
        static STORE: OnceCell<Mutex<HashMap<String, Arc<InnerFunction>>>> = OnceCell::new();
        let store = STORE.get_or_init(|| Mutex::new(HashMap::new()));
        InnerFunctionManager { store: store }
    }

    pub fn init(&mut self) {
        self.register(
            "min",
            Arc::new(|params| {
                let mut min = None;
                for param in params.into_iter() {
                    let num = param.decimal()?;
                    if min.is_none() || num < min.unwrap() {
                        min = Some(num);
                    }
                }
                if let Some(min) = min {
                    Ok(Value::Number(min))
                } else {
                    Err(Error::ParamEmpty("min".to_string()))
                }
            }),
        );

        self.register(
            "max",
            Arc::new(|params| {
                let mut max = None;
                for param in params.into_iter() {
                    let num = param.decimal()?;
                    if max.is_none() || num > max.unwrap() {
                        max = Some(num);
                    }
                }
                if let Some(max) = max {
                    Ok(Value::Number(max))
                } else {
                    Err(Error::ParamEmpty("max".to_string()))
                }
            }),
        );

        self.register(
            "sum",
            Arc::new(|params| {
                let mut ans = Decimal::ZERO;
                for param in params.into_iter() {
                    ans += param.decimal()?;
                }
                Ok(Value::Number(ans))
            }),
        );

        self.register(
            "mul",
            Arc::new(|params| {
                let mut ans = Decimal::ONE;
                for param in params.into_iter() {
                    ans *= param.decimal()?;
                }
                Ok(Value::Number(ans))
            }),
        );
    }

    pub fn register(&mut self, name: &str, f: Arc<InnerFunction>) {
        self.store.lock().unwrap().insert(name.to_string(), f);
    }

    pub fn get(&self, name: &str) -> Result<Arc<InnerFunction>> {
        let binding = self.store.lock().unwrap();
        let ans = binding.get(name);
        if ans.is_none() {
            return Err(Error::InnerFunctionNotRegistered(String::from(name)));
        }
        Ok(ans.unwrap().clone())
    }
}
