mod ast;
mod define;
mod error;
mod function;
mod keyword;
mod operator;
mod token;
mod tokenizer;
use rust_decimal::prelude::*;
use std::sync::Arc;

pub fn execute(expr: &str, ctx: define::Context) -> define::Result<define::Param> {
    ast::AST::new(expr)?.parse_expression()?.exec(Arc::new(ctx))
}

pub type Context = define::Context;

pub type Param = define::Param;

#[test]
fn test_exec() {
    let input = "(3+4)*5+mm*2";
    let mut ctx = Context::new();
    ctx.set_variable(&String::from("mm"), Param::Number(Decimal::new(2, 1)));
    match execute(input, ctx) {
        Err(e) => println!("{}", e),
        Ok(param) => println!("ans is {}", param),
    }
}
