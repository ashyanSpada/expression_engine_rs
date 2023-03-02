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

pub fn execute(expr: &str, mut ctx: context::Context) -> define::Result<value::Value> {
    ast::AST::new(expr)?.parse_expression()?.exec(&mut ctx)
}

pub type Value = value::Value;
pub type Context = context::Context;

#[test]
fn test_exec() {
    let input = "[]";
    let mut ctx = Context::new();
    ctx.set_variable(&String::from("mm"), Value::from(0.2));
    match execute(input, ctx) {
        Err(e) => println!("{}", e),
        Ok(param) => println!("ans is {}", param),
    }
}
