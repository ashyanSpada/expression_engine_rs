# expression_engine_rs Codebase Overview

This repository contains `expression_engine`, a Rust library for parsing and executing configurable expressions (useful for rules/business logic).

## Key technologies

- **Rust (edition 2021)** — core implementation.
- **`rust_decimal`** — decimal-based numeric model used by the runtime `Value::Number`.
- **`once_cell`** — global, lazily initialized registries (operators/functions/init guards).
- **`rstest`** — parameterized unit tests.
- **`criterion`** — benchmarks in `benches/`.

## High-level architecture

Expression execution pipeline:

1. **Tokenize** input string into lexical tokens.
2. **Parse** tokens into `ExprAST`.
3. **Execute** AST against a mutable `Context`.
4. **Dispatch** operators/functions through registries (built-in + user-registered).

Primary API entrypoints are in `src/lib.rs`:

- `execute(expr, ctx)`
- `parse_expression(expr)`
- `register_function(...)`
- `register_prefix_op(...)`
- `register_postfix_op(...)`
- `register_infix_op(...)`
- `create_context!` macro (from `src/context.rs`)

## Source tree organization

### Core runtime and API

- `src/lib.rs`  
  Public API surface and crate-level docs; initializes registries and forwards to parser/runtime.

- `src/context.rs`  
  Runtime symbol table (`Context`) for variables/functions backed by `Arc<Mutex<HashMap<...>>>`; includes `create_context!` macro.

- `src/value.rs`  
  Runtime value model: `String`, `Number`, `Bool`, `List`, `Map`, `None`, plus conversion helpers.

- `src/error.rs` / `src/define.rs`  
  Error enum and `Result<T>` alias used crate-wide.

### Language front-end (parsing)

- `src/tokenizer.rs`  
  Lexical scanner turning raw text into `Token`s.

- `src/token.rs`  
  Token/Span definitions and token utility methods.

- `src/parser.rs`  
  `ExprAST` definition, expression parser (precedence/associativity aware), AST display/description, and AST execution logic.

### Language semantics and extensibility

- `src/operator.rs`  
  Prefix/infix/postfix operator managers, built-in operator registration, precedence/associativity metadata.

- `src/function.rs`  
  Built-in function registry (`min`, `max`, `sum`, `avg`, `len`, `concat`, `first`, `keys`, etc.) and lookup/register APIs.

- `src/keyword.rs`  
  Operator classification helpers used by tokenizer/parser.

- `src/init.rs`  
  One-time initialization for built-in operators/functions.

### AST descriptor customization

- `src/descriptor.rs`  
  Descriptor manager for custom AST string descriptions (`ExprAST::describe` customization hooks).

## Tests, benchmarks, and CI

- Unit tests are colocated in each `src/*.rs` module.
- Doc tests run from API docs in `src/lib.rs` and macro docs in `src/context.rs`.
- Benchmarks:
  - `benches/execute_expression.rs`
  - `benches/display_expression.rs`
- GitHub Actions (`.github/workflows/`) cover multi-platform tests, formatting checks, clippy analysis, and coverage.

## Suggested reading order for new contributors

1. `src/lib.rs` (public API)
2. `src/value.rs` + `src/context.rs` (runtime data model)
3. `src/tokenizer.rs` + `src/token.rs` (lexing)
4. `src/parser.rs` (AST + parse + execute flow)
5. `src/operator.rs` + `src/function.rs` (semantic dispatch/extensibility)

## Useful local commands

- `cargo test`
- `cargo build`
- `cargo clippy --all-targets`
- `cargo bench`
