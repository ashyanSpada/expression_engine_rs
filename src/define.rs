use crate::error::Error;
use crate::function::InnerFunction;
use core::clone::Clone;
use core::hash::Hash;
use core::result;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Hash, PartialEq)]
pub enum Param {
    String(String),
    Number(Decimal),
    Bool(bool),
    List(Vec<Param>),
    Map(Vec<(Param, Param)>),
    None,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(val) => write!(f, "param string: {}", val.clone()),
            Self::Number(val) => write!(f, "param literal: {}", val.clone()),
            Self::Bool(val) => write!(f, "param bool: {}", val.clone()),
            Self::List(params) => {
                let mut s = String::from("[");
                for param in params {
                    s.push_str(format!("{},", param.clone()).as_str());
                }
                s.push_str("]");
                write!(f, "param list: {}", s)
            }
            Self::Map(m) => {
                let mut s = String::from("{");
                for (k, v) in m {
                    s.push_str(format!("key: {},", k.clone()).as_str());
                    s.push_str(format!("value: {}; ", v.clone()).as_str());
                }
                s.push_str("}");
                write!(f, "param map: {}", s)
            }
            Self::None => write!(f, "None"),
        }
    }
}

#[derive(Clone)]
pub enum ContextValue {
    Variable(Param),
    Function(Arc<InnerFunction>),
}

pub struct Context(pub HashMap<String, ContextValue>);

impl Context {
    pub fn new() -> Self {
        Context(HashMap::new())
    }

    pub fn set_func(&mut self, name: &String, func: Arc<InnerFunction>) {
        self.0
            .insert(name.clone(), ContextValue::Function(func.clone()));
    }

    pub fn set_variable(&mut self, name: &String, value: Param) {
        self.0.insert(name.clone(), ContextValue::Variable(value));
    }

    pub fn get_func(&self, name: &String) -> Option<Arc<InnerFunction>> {
        let value = self.0.get(name)?;
        match value {
            ContextValue::Function(func) => Some(func.clone()),
            ContextValue::Variable(_) => None,
        }
    }

    pub fn get_variable(&self, name: &String) -> Option<Param> {
        let value = self.0.get(name)?;
        match value {
            ContextValue::Variable(v) => Some(v.clone()),
            ContextValue::Function(_) => None,
        }
    }

    pub fn get(&self, name: &String) -> Option<ContextValue> {
        let value = self.0.get(name)?;
        Some(value.clone())
    }
}
