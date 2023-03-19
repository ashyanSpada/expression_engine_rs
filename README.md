# expression_engine_rs

## Introduction

Expression engine is a library written in pure Rust which provides an engine to compile and execute expressions. An expression indicates a string-like sentence that can be executed with some contexts and return a value (mostly, but not limited to, boolean, string and number).

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

+ Easy to Use (three lines at least)
+ Abundant Types and Expressions (Five fundamental types and seven kinds of expressions)
+ Pre-defined Operators Support (Common boolean, numeric and string operators)
+ Support function and operators registration
+ Support operator redirection

## Fundamental Types

The engine supports 5 fundamental types, respectively, the Boolean type, the Numeric type, the String type, the Map type and the List type.

| Type   | Example                  |
| ------ | ------------------------ |
| Bool   | True\|False\|true\|false |
| Number | 1.23\|-0.5\|1e3          |
| String | 'test'\|"test"           |
| Map    | {"a":"b","c":true}       |
| List   | [1,2,3,true,"res"]       |

## Definition

### Expression

```
Syntax
Expression(;Expression)*

Expression:
(
	LiteralExpression 
	| UnaryExpression
	| BinaryExpression
	| TernaryExpression
	| FunctionExpression
	| ReferenceExpression
	| ListExpression
	| MapExpression
	| NoneExpression

)

```

### LiteralExpression

```
Syntax
LiteralExpression:
  (LITERAL_NUMBER|LITERAL_BOOL|LITERAL_STRING)

```

A literal expression is an expression consisting of only one single token instead of a sequence of tokens. Here are 3 kinds of literal expresions, respectively, the LITERAL_NUMBER, the LITERAL_BOOL, and the LITERAL_STRING.

#### LITERAL_NUMBER

#### LITERAL_BOOL

The 'false' and 'False' will be parsed to the bool value **false**, while the 'true' and 'True' will be decoded to the bool value **true**.

#### LITERAL_STRING

A sequence of characters that starts with " and ends with " or starts with ' and ends with ' will be decoded as a LITERAL_STRING.

### UnaryExpression

```
Syntax
UnaryExpression:
  Operator Operand

Operand:
  Expression
```

A unary expression is consisted of an operand and a unary operator. All the unary operators have the same precedence and the right-to-left associativity.

| UnaryOp | Desc                      |
| ------- | ------------------------- |
| !       | Logical negation operator |
| not     | Logical negation operator |

### BinaryExpression

```
Syntax
BinaryExpression:
  Lhs Operator Rhs

Lhs:
  Expresssion

Rhs:
  Expression

```

### TernaryExpression

```
Syntax
TernaryExpression:
  Condition ? Lhs : Rhs

Condition:
  Expression

Lhs:
  Expression

Rhs:
  Expression
```

A binary expression contains two operands separated by an operator. All the binary operators have right-to-left associativity while their precedences may be not the same.

### FunctionExpression

```
Syntax
FunctionExpression:
  func(FunctionParams?)

FunctionParams:
  Expression(,Expression)*


```

### ReferenceExpression

### ListExpression

```
Syntax
ListExpression:
  [ListElements?]

ListElements:
  Expression(,Expression)*
```

### MapExpression

```
Syntax
MapExpression:
  {MapElements?}

MapElements:
  Expression:Expresssion(,Expression:Expression)*
```

### NoneExpression

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
