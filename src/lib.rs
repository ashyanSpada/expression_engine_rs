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
mod ast;
mod define;
mod error;
#[macro_use]
mod function;
mod keyword;
mod operator;
mod token;
mod tokenizer;
#[macro_use]
mod value;
mod context;

/// ## Usage
///
/// Calling the engine is simple. At first, define the expression you want to execute. Secondly, create a context to cache the pre-defined inner functions and variables. And then, register the variables and functions to the context. Finally, call the execute function with  the expression and context to get the executing result.
///
/// ``` rust
/// use expression_engine::Value;
/// use expression_engine::Context;
/// use expression_engine::execute;
/// let input = "(3+4)*5+mm*2";
/// let mut ctx = Context::new();
/// ctx.set_variable("mm", Value::from(2));
/// match execute(input, ctx) {
///     Err(e) => println!("{}", e),
///     Ok(param) => println!("ans is {}", param),
/// };
/// ```
pub fn execute(expr: &str, mut ctx: context::Context) -> define::Result<value::Value> {
    ast::AST::new(expr)?
        .parse_chain_expression()?
        .exec(&mut ctx)
}

pub type Value = value::Value;
pub type Context = context::Context;

#[test]
fn test_exec() {
    let input = "c = 5+3; c>>=10; c";
    let mut ctx = Context::new();
    ctx.set_variable(&String::from("mm"), Value::from(0.2));
    match execute(input, ctx) {
        Err(e) => println!("{}", e),
        Ok(param) => println!("ans is {}", param),
    }
}
