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
use std::sync::Arc;

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
pub fn execute(expr: &str, mut ctx: context::Context) -> Result<Value> {
    parse_expression(expr)?.exec(&mut ctx)
}

/// ## Usage
///
/// You can easily parse a string into ExprAST via this method.
///
/// ``` rust
/// use expression_engine::parse_expression;
/// let input = "a + 3*2+test()+[1,2,3,'haha']";
/// let ast = parse_expression(input);
/// assert!(ast.is_ok());
/// ```
pub fn parse_expression(expr: &str) -> Result<ExprAST> {
    init();
    parser::Parser::new(expr)?.parse_stmt()
}

/// ## Usage
///
/// You can register some inner functions in advance via this method
///
/// ``` rust
/// use std::sync::Arc;
/// use expression_engine::{register_function, create_context, execute, Value};
/// register_function("test", Arc::new(|_| return Ok(Value::from("test"))));
/// let input = "test()";
/// let ans = execute(input, create_context!());
/// assert!(ans.is_ok());
/// assert_eq!(ans.unwrap(), Value::from("test"));
/// ```
pub fn register_function(name: &str, handler: Arc<function::InnerFunction>) {
    use crate::function::InnerFunctionManager;
    init();
    InnerFunctionManager::new().register(name, handler);
}

/// ## Usage
///
/// You can register some prefix operators in advance via this method
///
/// ``` rust
/// use std::sync::Arc;
/// use expression_engine::{register_prefix_op, create_context, execute, Value};
/// register_prefix_op(
///      "+++",
///      Arc::new(|v| {
///          let mut tmp = v.integer()?;
///          tmp += 3;
///          Ok(Value::from(tmp))
///      }),
/// );
/// let input = "+++11";
/// let ans = execute(input, create_context!());
/// assert!(ans.is_ok());
/// assert_eq!(ans.unwrap(), Value::from(14));
/// ```
pub fn register_prefix_op(op: &str, handler: Arc<operator::PrefixOpFunc>) {
    use crate::operator::PrefixOpManager;
    init();
    PrefixOpManager::new().register(op, handler);
}

/// ## Usage
///
/// You can register some postfix operators in advance via this method
///
/// ``` rust
/// use std::sync::Arc;
/// use expression_engine::{register_postfix_op, create_context, execute, Value};
/// register_postfix_op(
///     "---",
///     Arc::new(|v| {
///         let mut tmp = v.integer()?;
///         tmp -= 3;
///         Ok(Value::from(tmp))
///     }),
/// );
/// let input = "100---";
/// let ans = execute(input, create_context!());
/// assert!(ans.is_ok());
/// assert_eq!(ans.unwrap(), Value::from(97));
/// ```
pub fn register_postfix_op(op: &str, handler: Arc<operator::PostfixOpFunc>) {
    use crate::operator::PostfixOpManager;
    init();
    PostfixOpManager::new().register(op, handler);
}

/// ## Usage
///
/// You can register some infix operators in advance via this method
///
/// ``` rust
/// use std::sync::Arc;
/// use expression_engine::{register_infix_op, create_context, execute, Value, InfixOpAssociativity, InfixOpType,};
/// register_infix_op(
///     "---",
///     100,
///    InfixOpType::CALC,
///    InfixOpAssociativity::RIGHT,
///    Arc::new(|left, right| Ok(Value::from(left.integer()? - right.integer()?))),
/// );
/// let input = "100---55---44";
/// let ans = execute(input, create_context!());
/// assert!(ans.is_ok());
/// assert_eq!(ans.unwrap(), Value::from(89));
/// ```
pub fn register_infix_op(
    op: &str,
    precedence: i32,
    op_type: InfixOpType,
    associativity: InfixOpAssociativity,
    handler: Arc<operator::InfixOpFunc>,
) {
    use crate::operator::InfixOpManager;
    init();
    InfixOpManager::new().register(op, precedence, op_type, associativity, handler);
}

fn init() {
    use crate::init::init;
    init();
}

pub type Value = value::Value;
pub type Context = context::Context;
pub type Result<T> = define::Result<T>;
pub type ExprAST<'a> = parser::ExprAST<'a>;
pub type InfixOpType = operator::InfixOpType;
pub type InfixOpAssociativity = operator::InfixOpAssociativity;

#[cfg(test)]
mod tests {
    use crate::{
        create_context, execute, parse_expression, register_function, register_infix_op,
        register_postfix_op, register_prefix_op, InfixOpAssociativity, InfixOpType, Value,
    };
    use std::sync::Arc;
    #[test]
    fn test_execute() {
        let input = "c = 5+3; c+=10+f; c";
        let ctx = create_context!(
            "d" => 2,
            "b" => true,
            "f" => Arc::new(|_| Ok(Value::from(3)))
        );
        let ans = execute(input, ctx).unwrap();
        assert_eq!(ans, 21.into())
    }

    #[test]
    fn test_parse_expression() {
        let input = "a + 3*2+test()+[1,2,3,'haha']";
        assert!(parse_expression(input).is_ok());
    }

    #[test]
    fn test_register_function() {
        register_function("test", Arc::new(|_| return Ok(Value::from("test"))));
        let input = "test()";
        let ans = execute(input, create_context!());
        assert!(ans.is_ok());
        assert_eq!(ans.unwrap(), Value::from("test"));
    }

    #[test]
    fn test_register_prefix_op() {
        register_prefix_op(
            "+++",
            Arc::new(|v| {
                let mut tmp = v.integer()?;
                tmp += 3;
                Ok(Value::from(tmp))
            }),
        );
        let input = "+++11";
        let ans = execute(input, create_context!());
        assert!(ans.is_ok());
        assert_eq!(ans.unwrap(), Value::from(14));
    }

    #[test]
    fn test_register_postfix_op() {
        register_postfix_op(
            "---",
            Arc::new(|v| {
                let mut tmp = v.integer()?;
                tmp -= 3;
                Ok(Value::from(tmp))
            }),
        );
        let input = "100---";
        let ans = execute(input, create_context!());
        assert!(ans.is_ok());
        assert_eq!(ans.unwrap(), Value::from(97));
    }

    #[test]
    fn test_register_infix_op() {
        register_infix_op(
            "right_minus",
            100,
            InfixOpType::CALC,
            InfixOpAssociativity::RIGHT,
            Arc::new(|left, right| Ok(Value::from(left.integer()? - right.integer()?))),
        );
        let input = "100 right_minus 55 right_minus 44";
        let ans = execute(input, create_context!());
        match &ans {
            Ok(_) => {}
            Err(e) => print!("err is {}", e),
        }
        assert!(ans.is_ok());
        assert_eq!(ans.unwrap(), Value::from(89));
    }
}
