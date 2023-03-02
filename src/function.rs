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
        let store = STORE.get_or_init(|| Mutex::new(Self::internal_register(HashMap::new())));
        InnerFunctionManager { store: store }
    }

    fn internal_register(
        mut m: HashMap<String, Arc<InnerFunction>>,
    ) -> HashMap<String, Arc<InnerFunction>> {
        m.insert(
            "min".to_string(),
            Arc::new(|params| {
                let mut min = None;
                for param in params.into_iter() {
                    match param {
                        Value::Number(num) => {
                            if min.is_none() || num < min.unwrap() {
                                min = Some(num);
                            }
                        }
                        _ => return Err(Error::ShouldBeNumber()),
                    }
                }
                Ok(Value::Number(min.unwrap()))
            }),
        );

        m.insert(
            "max".to_string(),
            Arc::new(|params| {
                let mut max = None;
                for param in params.into_iter() {
                    match param {
                        Value::Number(num) => {
                            if max.is_none() || num > max.unwrap() {
                                max = Some(num);
                            }
                        }
                        _ => return Err(Error::ShouldBeNumber()),
                    }
                }
                Ok(Value::Number(max.unwrap()))
            }),
        );

        m.insert(
            "sum".to_string(),
            Arc::new(|params| {
                let mut ans = Decimal::ZERO;
                for param in params.into_iter() {
                    match param {
                        Value::Number(num) => {
                            ans += num;
                        }
                        _ => return Err(Error::ShouldBeNumber()),
                    }
                }
                Ok(Value::Number(ans))
            }),
        );

        m.insert(
            "mul".to_string(),
            Arc::new(|params| {
                let mut ans = Decimal::ONE;
                for param in params.into_iter() {
                    match param {
                        Value::Number(num) => {
                            ans *= num;
                        }
                        _ => return Err(Error::ShouldBeNumber()),
                    }
                }
                Ok(Value::Number(ans))
            }),
        );
        m
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

#[macro_export]
macro_rules! func {
    ($func:expr) => {
        Arc::new($func)
    };
}

#[test]
fn test_register() {
    use crate::func;
    let mut m = InnerFunctionManager::new();
    m.register(
        "test",
        func!(|params| {
            let mut ans = Decimal::ZERO;
            for param in params.into_iter() {
                match param {
                    Value::Number(num) => {
                        ans += num;
                    }
                    _ => return Err(Error::ShouldBeNumber()),
                }
            }
            Ok(Value::Number(ans))
        }),
    );
}
