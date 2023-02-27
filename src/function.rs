use crate::define::{Param, Result};
use crate::error::Error;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::sync::Arc;

pub type InnerFunction = dyn Fn(Vec<Param>) -> Result<Param> + Send + 'static;

pub struct InnerFunctionManager {
    pub store: HashMap<String, Arc<InnerFunction>>,
}

impl InnerFunctionManager {
    pub fn new() -> Arc<Self> {
        static mut INNER_FUNCTION_MANAGER: Option<Arc<InnerFunctionManager>> = None;
        unsafe {
            match &INNER_FUNCTION_MANAGER {
                Some(m) => m.clone(),
                None => INNER_FUNCTION_MANAGER
                    .get_or_insert(Arc::new(InnerFunctionManager {
                        store: Self::internal_register(HashMap::new()),
                    }))
                    .clone(),
            }
        }
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
                        Param::Number(num) => {
                            if min.is_none() || num < min.unwrap() {
                                min = Some(num);
                            }
                        }
                        _ => return Err(Error::ShouldBeNumber()),
                    }
                }
                Ok(Param::Number(min.unwrap()))
            }),
        );

        m.insert(
            "max".to_string(),
            Arc::new(|params| {
                let mut max = None;
                for param in params.into_iter() {
                    match param {
                        Param::Number(num) => {
                            if max.is_none() || num > max.unwrap() {
                                max = Some(num);
                            }
                        }
                        _ => return Err(Error::ShouldBeNumber()),
                    }
                }
                Ok(Param::Number(max.unwrap()))
            }),
        );

        m.insert(
            "sum".to_string(),
            Arc::new(|params| {
                let mut ans = Decimal::ZERO;
                for param in params.into_iter() {
                    match param {
                        Param::Number(num) => {
                            ans += num;
                        }
                        _ => return Err(Error::ShouldBeNumber()),
                    }
                }
                Ok(Param::Number(ans))
            }),
        );

        m.insert(
            "mul".to_string(),
            Arc::new(|params| {
                let mut ans = Decimal::ONE;
                for param in params.into_iter() {
                    match param {
                        Param::Number(num) => {
                            ans *= num;
                        }
                        _ => return Err(Error::ShouldBeNumber()),
                    }
                }
                Ok(Param::Number(ans))
            }),
        );
        m
    }

    pub(crate) fn register(&mut self, name: &str, f: Arc<InnerFunction>) {
        self.store.insert(String::from(name), f);
    }

    pub fn get(&self, name: &str) -> Result<Arc<InnerFunction>> {
        let ans = self.store.get(name);
        if ans.is_none() {
            return Err(Error::InnerFunctionNotRegistered(String::from(name)));
        }
        Ok(ans.unwrap().clone())
    }
}
