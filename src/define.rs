use crate::error::Error;
use core::result;

pub type Result<T> = result::Result<T, Error>;