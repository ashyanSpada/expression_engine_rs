# expression_engine
[![Build Status](https://github.com/ashyanSpada/expression_engine_rs/workflows/Build/badge.svg)](https://github.com/ashyanSpada/expression_engine_rs/actions)
[![Latest Version](https://img.shields.io/crates/v/expression_engine.svg)](https://crates.io/crates/expression_engine)
[![License:Apache-2.0](https://img.shields.io/badge/License-Apache2.0-yellow.svg)](https://opensource.org/licenses/Apache-2.0)
[![codecov](https://codecov.io/gh/ashyanSpada/expression_engine_rs/graph/badge.svg?token=H5GNNRVUZQ)](https://codecov.io/gh/ashyanSpada/expression_engine_rs)

## Introduction

Expression engine is a library written in pure Rust which provides an engine to compile and execute expressions. An expression indicates a string-like sentence that can be executed with some contexts and return a value (mostly, but not limited to, boolean, string and number).

Expression engine aims to provide an engine for users that can execute complex logics using configurations without recompiling. It's a proper alternative as the basis to build business rule engines.

## Usage

Calling the engine is simple. At first, define the expression you want to execute. Secondly, create a context to cache the pre-defined inner functions and variables. And then, register the variables and functions to the context. Finally, call the execute function with  the expression and context to get the executing result.

```rust
use expression_engine::{create_context, execute, Value};
let input = "c = 5+3; c+=10+f; c";
let ctx = create_context!(
     "d" => 2,
     "b" => true,
     "f" => Arc::new(|params| Ok(Value::from(3)))
);
let ans = execute(input, ctx).unwrap();
assert_eq!(ans, Value::from(21))
```

## Features

+ Easy to Use (three lines at least)
+ Abundant Types and Expressions (Five fundamental types and seven kinds of expressions)
+ Pre-defined Operators Support (Common boolean, numeric and string operators)
+ Support function and operators registration
+ Support operator redirection

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

```rust
fn is_digit_char(ch: char) -> bool {
    return '0' <= ch && ch <= '9' || ch == '.' || ch == '-' || ch == 'e' || ch == 'E' || ch == '+';
}
```

Continuous chars with patterns as above will be parsed to a number.

#### LITERAL_BOOL

The `false` and `False` will be parsed to the bool value **false**, while the `true` and `True` will be decoded to the bool value **true**.

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

A binary expression contains two operands separated by an operator. All the binary operators have right-to-left associativity while their precedences may be not the same. The supported binary operators are as below:

| Operator  | Precedence | Desc |
| --------- | ---------- | ---- |
| \|=       | 20         |      |
| <<=       | 20         |      |
| =         | 20         |      |
| +=        | 20         |      |
| ^=        | 20         |      |
| /=        | 20         |      |
| &=        | 20         |      |
| >>=       | 20         |      |
| %=        | 20         |      |
| -=        | 20         |      |
| *=        | 20         |      |
| \|\|      | 40         |      |
| &&        | 50         |      |
| >         | 60         |      |
| >=        | 60         |      |
| <=        | 60         |      |
| !=        | 60         |      |
| <         | 60         |      |
| ==        | 60         |      |
| \|        | 70         |      |
| ^         | 80         |      |
| &         | 90         |      |
| >>        | 100        |      |
| <<        | 100        |      |
| +         | 110        |      |
| -         | 110        |      |
| *         | 120        |      |
| %         | 120        |      |
| /         | 120        |      |
| beginWith | 200        | Returns true if the left string starts with the right string |
| endWith   | 200        | Returns true if the left string ends with the right string   |
| in        | 200        | Returns true if the left value is found in the right list    |
| notIn     | 200        | Returns true if the left value is not found in the right list |
| contains  | 200        | Returns true if the left string contains the right string    |

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

The ternary expression is composed of three parts, respectively the Condition, the Lhs and the Rhs. If the result of the Condition is true, then return to Lhs. Otherwise the result of Rhs is returned.

### FunctionExpression

```
Syntax
FunctionExpression:
  func(FunctionParams?)

FunctionParams:
  Expression(,Expression)*


```

The function name with the params which are a sequence of expressions separated by comma consist the function expression.

### ReferenceExpression

The reference expression is either a variable or a function with no params.

### ListExpression

```
Syntax
ListExpression:
  [ListElements?]

ListElements:
  Expression(,Expression)*
```

The list expression starts with the open bracket and ends with the close bracket. Its params are a list of expressions.

### MapExpression

```
Syntax
MapExpression:
  {MapElements?}

MapElements:
  KeyElement:ValueElement(,KeyElement:ValueElement)*

KeyElement:
  Expression

ValueElement:
  Expression
```

The map expression begins with the open brace and ends with the close brace with a sequence of k, v pair where both the k and v are expressions.

### NoneExpression

```
Syntax
NoneExpression:
  None
  
```

The return value of the NoneExpression is `None`.

## Built-in Functions

The following functions are available without any registration:

### Math Functions

| Function       | Signature          | Description                                         |
| -------------- | ------------------ | --------------------------------------------------- |
| min            | min(x, ...)        | Returns the minimum value among all numeric params  |
| max            | max(x, ...)        | Returns the maximum value among all numeric params  |
| sum            | sum(x, ...)        | Returns the sum of all numeric params               |
| mul            | mul(x, ...)        | Returns the product of all numeric params           |
| avg            | avg(x, ...)        | Returns the average of all numeric params           |
| abs            | abs(x)             | Returns the absolute value of x                     |
| floor          | floor(x)           | Rounds x down to the nearest integer                |
| ceil           | ceil(x)            | Rounds x up to the nearest integer                  |
| round          | round(x)           | Rounds x to the nearest integer                     |
| pow            | pow(base, exp)     | Returns base raised to the power exp                |
| sqrt           | sqrt(x)            | Returns the square root of x                        |

### String Functions

| Function       | Signature          | Description                                         |
| -------------- | ------------------ | --------------------------------------------------- |
| len            | len(x)             | Returns the length of a string, list, or map        |
| upper          | upper(s)           | Converts string s to uppercase                      |
| lower          | lower(s)           | Converts string s to lowercase                      |
| trim           | trim(s)            | Removes leading and trailing whitespace from s      |
| concat         | concat(s, ...)     | Concatenates all string params                      |
| str            | str(x)             | Converts a value to its string representation       |
| num            | num(x)             | Converts a string or bool to a number               |

### List Functions

| Function       | Signature          | Description                                         |
| -------------- | ------------------ | --------------------------------------------------- |
| first          | first(list)        | Returns the first element of the list               |
| last           | last(list)         | Returns the last element of the list                |

### Map Functions

| Function       | Signature          | Description                                         |
| -------------- | ------------------ | --------------------------------------------------- |
| keys           | keys(map)          | Returns a list of all keys in the map               |
| values         | values(map)        | Returns a list of all values in the map             |
