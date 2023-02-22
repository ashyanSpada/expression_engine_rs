# expression_executor_rs

## Introduction

Expression executor is a library written in pure Rust which provides an engine to compile and execute expressions. An expression indicates a string-like sentence that can be executed with some contexts and return a value (mostly, but not limited to, boolean, string and number).

Expression executor aims to provide an engine for users that can execute complex logics using configurations without recompiling. It's a proper alternative as the basis to build business rule engines.

## Usage

Calling the engine is simple. At first, define the expression you want to execute. Secondly, create a context to cache the pre-defined inner functions and variables. And then, register the variables and functions to the context. Finally, call the execute function with  the expression and context to get the executing result.

```rust
let input = "(3+4)*5+mm*2";  // the input is an expression
let mut ctx = Context::new(); // create a context
ctx.set_variable(&String::from("mm"), Param::Number(Decimal::new(2, 1)));
match execute(input, ctx) {
    Err(e) => println!("{}", e),
    Ok(param) => println!("ans is {}", param),
}
```

## Features

## Fundamental Types

The engine supports 5 fundamental types, respectively, the Boolean type, the Numeric type, the String type, the Map type and the List type.

| Type   | Example                  |
| ------ | ------------------------ |
| Bool   | True\|False\|true\|false |
| Number | 1.23\|-0.5\|1e3          |
| String | 'test'\|"test"           |
| Map    | {"a":"b","c":true}       |
| List   | [1,2,3,true,"res"]       |

## Supported Exprs

### Unary Expr

```
Pattern: op expr
Example: !(2 > 3)
```

| Op  | Desc |
| --- | ---- |
| !   |      |
| not |      |

### Binary Expr

```
Pattern: expr1 op expr2
Example: (2 + 3) * 5
```

| Op        | Precedence | Desc |
| --------- | ---------- | ---- |
| !=        | 20         |      |
| ==        | 20         |      |
| >         | 20         |      |
| <         | 20         |      |
| >=        | 20         |      |
| <=        | 20         |      |
| \|\|      | 40         |      |
| &&        | 40         |      |
| +         | 60         |      |
| -         | 60         |      |
| *         | 80         |      |
| /         | 80         |      |
| %         | 80         |      |
| in        | 100        |      |
| endWith   | 120        |      |
| beginWith | 120        |      |

### Ternary Expr

```
Pattern: expr1 ? expr2 : expr3
Example: (2>3) ? true : 'test'
```

### Function

```
Pattern: func(expr1, expr2, ...)
Example: max(1, 2+3, (4+5)*2)
```

### Reference

`Pattern: a + b`

Example:  a + b

### List

```
Pattern: [expr1, expr2, expr3, ...]
Example: [a, 1, 2+3, true, "test"]
```

### Map

```
Pattern: {expr1: expr2, expr3: expr4, ...}
Example: {1:2, "key": "test", true: false}
```
