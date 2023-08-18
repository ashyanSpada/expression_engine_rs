//! Expression engine is a library written in pure Rust which provides an engine to compile and execute expressions.
//! An expression indicates a string-like sentence that can be executed with some contexts and return a value (mostly, but not limited to, boolean, string and number).
//! Expression engine aims to provide an engine for users that can execute complex logics using configurations without recompiling.
//! It's a proper alternative as the basis to build business rule engines.
//! ## Features

//! + Easy to Use (three lines at least)
//! + Abundant Types and Expressions (Five fundamental types and seven kinds of expressions)
//! + Pre-defined Operators Support (Common boolean, numeric and string operators)
//! + Support function and operators registration
//! + Support operator redirection
mod define;
mod error;
mod parser;
#[macro_use]
mod function;
mod keyword;
mod operator;
mod token;
mod tokenizer;
#[macro_use]
mod value;
mod context;
mod descriptor;
mod init;

/// ## Usage
///
/// Calling the engine is simple. At first, define the expression you want to execute. Secondly, create a context to cache the pre-defined inner functions and variables. And then, register the variables and functions to the context. Finally, call the execute function with  the expression and context to get the executing result.
///
/// ``` rust
/// use expression_engine::{create_context, execute, Value};
/// let input = "c = 5+3; c+=10+f; c";
/// let ctx = create_context!(
///     "d" => 2,
///     "b" => true,
///     "f" => Arc::new(|params| Ok(Value::from(3)))
/// );
/// let ans = execute(input, ctx).unwrap();
/// assert_eq!(ans, Value::from(21))
/// ```
pub fn execute(expr: &str, mut ctx: context::Context) -> define::Result<value::Value> {
    use crate::init::init;
    init();
    parser::Parser::new(expr)?
        .parse_chain_expression()?
        .exec(&mut ctx)
}

pub type Value = value::Value;
pub type Context = context::Context;

#[test]
fn test_exec() {
    let input = "c = 5+3; c+=10+f; c";
    let ctx = create_context!(
        "d" => 2,
        "b" => true,
        "f" => Arc::new(|_| Ok(Value::from(3)))
    );
    let ans = execute(input, ctx).unwrap();
    assert_eq!(ans, 21.into())
}
