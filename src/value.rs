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

// Optimization: Write directly to `std::fmt::Formatter` instead of creating intermediate
// strings and calling `format!()` in a loop.
// Impact: Reduces heap allocations, avoiding O(N) allocations for Lists and Maps.
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
                    write!(f, "key: {},value: {}; ", k, v)?;
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
}
