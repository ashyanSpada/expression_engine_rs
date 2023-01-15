use rust_decimal::Decimal;
use core::result;
use crate::error::Error;
use core::clone::Clone;
use std::fmt;
use core::hash::Hash;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Hash, PartialEq)]
pub enum Param {
    String(String),
    Literal(Decimal),
    Bool(bool),
    List(Vec<Param>),
    Map(Vec<(Param, Param)>)
}

// pub trait InnerFunction {
//     fn call(&self, param: Vec<Param>) -> Result<Param>;
// }

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(val) => write!(f, "param string: {}", val.clone()),
            Self::Literal(val) => write!(f, "param literal: {}", val.clone()),
            Self::Bool(val) => write!(f, "param bool: {}", val.clone()),
            Self::List(params) => {
                let mut s = String::from("[");
                for param in params {
                    s.push_str(format!("{},", param.clone()).as_str());
                }
                s.push_str("]");
                write!(f, "param list: {}", s)
            },
            Self::Map(m) => {
                let mut s = String::from("{");
                for (k, v) in m {
                    s.push_str(format!("key: {},", k.clone()).as_str());
                    s.push_str(format!("value: {}; ", v.clone()).as_str());
                }
                s.push_str("}");
                write!(f, "param map: {}", s)
            },
        }
    }
}