use crate::define::Result;
use crate::error::Error;
use rust_decimal::prelude::*;
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    String(String),
    Number(Decimal),
    Bool(bool),
    List(Vec<Value>),
    Map(Vec<(Value, Value)>),
    None,
}

#[cfg(not(tarpaulin_include))]
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(val) => write!(f, "value string: {}", val),
            Self::Number(val) => write!(f, "value number: {}", val),
            Self::Bool(val) => write!(f, "value bool: {}", val),
            Self::List(values) => {
                write!(f, "value list: [")?;
                for value in values {
                    write!(f, "{},", value)?;
                }
                write!(f, "]")
            }
            Self::Map(m) => {
                write!(f, "value map: {{")?;
                for (k, v) in m {
                    write!(f, "key: {},", k)?;
                    write!(f, "value: {}; ", v)?;
                }
                write!(f, "}}")
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

impl Value {
    pub fn decimal(self) -> Result<rust_decimal::Decimal> {
        match self {
            Self::Number(val) => Ok(val),
            _ => Err(Error::ShouldBeNumber()),
        }
    }

    pub fn string(self) -> Result<String> {
        match self {
            Self::String(val) => Ok(val),
            _ => Err(Error::ShouldBeString()),
        }
    }

    pub fn bool(self) -> Result<bool> {
        match self {
            Self::Bool(val) => Ok(val),
            _ => Err(Error::ShouldBeBool()),
        }
    }

    pub fn integer(self) -> Result<i64> {
        match self {
            Self::Number(val) => {
                if val.scale() == 0 {
                    val.to_i64().ok_or(Error::InvalidInteger)
                } else {
                    Err(Error::InvalidInteger)
                }
            }
            _ => Err(Error::InvalidInteger),
        }
    }

    pub fn float(self) -> Result<f64> {
        match self {
            Self::Number(val) => val.to_f64().ok_or(Error::InvalidFloat),
            _ => Err(Error::InvalidFloat),
        }
    }

    pub fn list(self) -> Result<Vec<Value>> {
        match self {
            Self::List(list) => Ok(list),
            _ => Err(Error::ShouldBeList()),
        }
    }

    pub fn map(self) -> Result<Vec<(Value, Value)>> {
        match self {
            Self::Map(m) => Ok(m),
            _ => Err(Error::ShouldBeMap()),
        }
    }
}

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

impl_value_from_for_number!(
    [i128, from_i128],
    [i32, from_i32],
    [i64, from_i64],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_integer() {
        assert_eq!(Value::from(10).integer().unwrap(), 10);
        assert_eq!(Value::from(-5).integer().unwrap(), -5);

        // Test that integer() fails when scale is not 0
        let dec = Decimal::from_str("10.5").unwrap();
        assert!(Value::Number(dec).integer().is_err());

        // Even if the value represents an integer, but has a non-zero scale, it should fail to parse
        // according to strict behavior checking `val.scale() == 0`
        let dec_with_scale = Decimal::from_str("10.0").unwrap();
        assert!(Value::Number(dec_with_scale).integer().is_err());
    }

    #[test]
    fn test_value_float() {
        assert_eq!(Value::from(10).float().unwrap(), 10.0);
        assert_eq!(Value::from(-5).float().unwrap(), -5.0);

        let dec = Decimal::from_str("10.5").unwrap();
        assert_eq!(Value::Number(dec).float().unwrap(), 10.5);

        let dec_with_scale = Decimal::from_str("10.0").unwrap();
        assert_eq!(Value::Number(dec_with_scale).float().unwrap(), 10.0);
    }

    #[test]
    fn test_value_display_string() {
        assert_eq!(
            format!("{}", Value::String("hello".into())),
            "value string: hello"
        );
    }

    #[test]
    fn test_value_display_number() {
        assert_eq!(format!("{}", Value::from(42i32)), "value number: 42");
    }

    #[test]
    fn test_value_display_bool() {
        assert_eq!(format!("{}", Value::Bool(true)), "value bool: true");
        assert_eq!(format!("{}", Value::Bool(false)), "value bool: false");
    }

    #[test]
    fn test_value_display_none() {
        assert_eq!(format!("{}", Value::None), "None");
    }

    #[test]
    fn test_value_display_list() {
        let list = Value::List(vec![Value::from(1i32), Value::from(2i32)]);
        assert_eq!(
            format!("{}", list),
            "value list: [value number: 1,value number: 2,]"
        );
    }

    #[test]
    fn test_value_display_list_empty() {
        assert_eq!(format!("{}", Value::List(vec![])), "value list: []");
    }

    #[test]
    fn test_value_display_map() {
        let map = Value::Map(vec![(Value::String("k".into()), Value::from(1i32))]);
        assert_eq!(
            format!("{}", map),
            "value map: {key: value string: k,value: value number: 1; }"
        );
    }

    #[test]
    fn test_value_display_map_empty() {
        assert_eq!(format!("{}", Value::Map(vec![])), "value map: {}");
    }

    #[test]
    fn test_value_map() {
        let pairs = vec![
            (Value::String("key".into()), Value::from(1i32)),
            (Value::String("other".into()), Value::from(2i32)),
        ];
        let v = Value::Map(pairs.clone());
        assert_eq!(v.map().unwrap(), pairs);
    }

    #[test]
    fn test_value_map_err() {
        assert!(Value::from(42i32).map().is_err());
        assert!(Value::from(true).map().is_err());
        assert!(Value::from("str").map().is_err());
        assert!(Value::List(vec![]).map().is_err());
        assert!(Value::None.map().is_err());
    }
}
