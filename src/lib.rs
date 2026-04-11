//! Expression engine is a library written in pure Rust which parses expressions into AST, compiles them into bytecode, and executes them.
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
pub mod bytecode;
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
    let ast = parse_expression(expr)?;
    let program = bytecode::compile_expression(&ast)?;
    bytecode::execute_program(&program, &mut ctx)
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
/// Parse an expression and compile it directly to bytecode without keeping the AST.
/// This is useful when you want to cache the compiled bytecode and execute it multiple times
/// with different contexts, avoiding the parsing and compilation overhead.
///
/// ``` rust
/// use expression_engine::{parse_expression_to_bytecode, execute_program, create_context, Value};
/// let input = "a + b * 2";
/// let program = parse_expression_to_bytecode(input).unwrap();
///
/// // Execute the same bytecode program multiple times with different contexts
/// let mut ctx1 = create_context!("a" => 10, "b" => 5);
/// let result1 = execute_program(&program, &mut ctx1).unwrap();
/// assert_eq!(result1, Value::from(20));
///
/// let mut ctx2 = create_context!("a" => 1, "b" => 2);
/// let result2 = execute_program(&program, &mut ctx2).unwrap();
/// assert_eq!(result2, Value::from(5));
/// ```
pub fn parse_expression_to_bytecode(expr: &str) -> Result<bytecode::Program> {
    init();
    let ast = parser::Parser::new(expr)?.parse_stmt()?;
    bytecode::compile_expression(&ast)
}

/// ## Usage
///
/// Execute a pre-compiled bytecode program with a given context.
/// This function should be used together with `parse_expression_to_bytecode` when you need
/// to execute the same expression multiple times with different contexts, which is more
/// efficient than calling `execute` repeatedly.
///
/// ``` rust
/// use expression_engine::{parse_expression_to_bytecode, execute_program, create_context, Value};
/// let input = "x * 2 + y";
/// let program = parse_expression_to_bytecode(input).unwrap();
///
/// let mut ctx = create_context!("x" => 5, "y" => 3);
/// let result = execute_program(&program, &mut ctx).unwrap();
/// assert_eq!(result, Value::from(13));
/// ```
pub fn execute_program(program: &bytecode::Program, ctx: &mut Context) -> Result<value::Value> {
    init();
    bytecode::execute_program(program, ctx)
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
pub type Error = error::Error;
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

    #[test]
    fn test_min_no_panic() {
        use crate::error::Error;
        let input = "min()";
        let ctx = create_context!();
        let ans = execute(input, ctx);
        assert!(ans.is_err());
        match ans.unwrap_err() {
            Error::ParamEmpty(name) => assert_eq!(name, "min"),
            _ => panic!("Expected Error::ParamEmpty"),
        }
    }

    #[test]
    fn test_max_no_panic() {
        use crate::error::Error;
        let input = "max()";
        let ctx = create_context!();
        let ans = execute(input, ctx);
        assert!(ans.is_err());
        match ans.unwrap_err() {
            Error::ParamEmpty(name) => assert_eq!(name, "max"),
            _ => panic!("Expected Error::ParamEmpty"),
        }
    }

    #[test]
    fn test_div_zero_no_panic() {
        use crate::error::Error;
        let input = "5 / 0";
        let ctx = create_context!();
        let ans = execute(input, ctx);
        assert!(ans.is_err());
        match ans.unwrap_err() {
            Error::DivByZero => (),
            _ => panic!("Expected Error::DivByZero"),
        }
    }

    #[test]
    fn test_mod_zero_no_panic() {
        use crate::error::Error;
        let input = "5 % 0";
        let ctx = create_context!();
        let ans = execute(input, ctx);
        assert!(ans.is_err());
        match ans.unwrap_err() {
            Error::DivByZero => (),
            _ => panic!("Expected Error::DivByZero"),
        }
    }

    #[test]
    fn test_parse_expression_to_bytecode() {
        let input = "a + b * 2";
        let program = crate::parse_expression_to_bytecode(input);
        assert!(program.is_ok());
        let program = program.unwrap();
        assert!(!program.instructions.is_empty());
        assert!(!program.constants.is_empty());
    }

    #[test]
    fn test_execute_program() {
        let input = "x * 2 + y";
        let program = crate::parse_expression_to_bytecode(input).unwrap();
        let mut ctx = create_context!("x" => 5, "y" => 3);
        let result = crate::execute_program(&program, &mut ctx).unwrap();
        assert_eq!(result, Value::from(13));
    }

    #[test]
    fn test_parse_and_execute_bytecode_reuse() {
        // Test that we can compile once and execute multiple times
        let input = "a + b * 2";
        let program = crate::parse_expression_to_bytecode(input).unwrap();

        // Execute with first context
        let mut ctx1 = create_context!("a" => 10, "b" => 5);
        let result1 = crate::execute_program(&program, &mut ctx1).unwrap();
        assert_eq!(result1, Value::from(20));

        // Execute with second context
        let mut ctx2 = create_context!("a" => 1, "b" => 2);
        let result2 = crate::execute_program(&program, &mut ctx2).unwrap();
        assert_eq!(result2, Value::from(5));

        // Execute with third context
        let mut ctx3 = create_context!("a" => 100, "b" => 50);
        let result3 = crate::execute_program(&program, &mut ctx3).unwrap();
        assert_eq!(result3, Value::from(200));
    }

    #[test]
    fn test_execute_program_with_function() {
        let input = "f(x) + y";
        let program = crate::parse_expression_to_bytecode(input).unwrap();
        let mut ctx = create_context!(
            "x" => 10,
            "y" => 5,
            "f" => Arc::new(|params| {
                let val = params[0].clone().integer()?;
                Ok(Value::from(val * 2))
            })
        );
        let result = crate::execute_program(&program, &mut ctx).unwrap();
        assert_eq!(result, Value::from(25));
    }

    #[test]
    fn test_parse_expression_to_bytecode_invalid() {
        let input = "a + )"; // Invalid expression - unmatched closing paren
        let result = crate::parse_expression_to_bytecode(input);
        assert!(result.is_err());
    }
}
