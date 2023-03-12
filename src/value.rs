use rust_decimal::prelude::*;
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(Decimal),
    Bool(bool),
    List(Vec<Value>),
    Map(Vec<(Value, Value)>),
    None,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(val) => write!(f, "value string: {}", val.clone()),
            Self::Number(val) => write!(f, "value number: {}", val.clone()),
            Self::Bool(val) => write!(f, "value bool: {}", val.clone()),
            Self::List(values) => {
                let mut s = String::from("[");
                for value in values {
                    s.push_str(format!("{},", value.clone()).as_str());
                }
                s.push_str("]");
                write!(f, "value list: {}", s)
            }
            Self::Map(m) => {
                let mut s = String::from("{");
                for (k, v) in m {
                    s.push_str(format!("key: {},", k.clone()).as_str());
                    s.push_str(format!("value: {}; ", v.clone()).as_str());
                }
                s.push_str("}");
                write!(f, "value map: {}", s)
            }
            Self::None => write!(f, "None"),
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::List(value)
    }
}

impl From<Decimal> for Value {
    fn from(value: Decimal) -> Self {
        Value::Number(value)
    }
}

use crate::impl_value_from_for_number;

impl_value_from_for_number!(
    [i128, from_i128],
    [i64, from_i64],
    [i32, from_i32],
    [i16, from_i16],
    [i8, from_i8],
    [u128, from_u128],
    [u64, from_u64],
    [u32, from_u32],
    [u16, from_u16],
    [u8, from_u8],
    [f64, from_f64],
    [f32, from_f32]
);

#[macro_export]
macro_rules! impl_value_from_for_number {
    ($([$number_type:tt, $method_name: ident]),+) => {
        $(
            impl From<$number_type> for Value {
                fn from(value: $number_type) -> Self {
                    Value::Number(Decimal::$method_name(value).unwrap_or_default())
                }
            }
        )+
    };
}
