use rust_decimal::Decimal;
use core::result;
use crate::error::Error;
use core::clone::Clone;
use std::fmt;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone)]
pub enum Param {
    String(String),
    Literal(Decimal),
    Bool(bool),
    // List(Vec<Param>)
}

pub trait InnerFunction {
    fn call(&self, param: Vec<Param>) -> Result<Param>;
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Param::String(val) => write!(f, "param string: {}", val.clone()),
            Param::Literal(val) => write!(f, "param literal: {}", val.clone()),
            Param::Bool(val) => write!(f, "param bool: {}", val.clone()),
        }
    }
}