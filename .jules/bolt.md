## 2024-05-24 - Avoid vec! for constant collections in initialization loops
**Learning:** Initializing maps/operator managers by iterating over `vec![...]` causes unnecessary heap allocations. Using array literals `[...]` is significantly more efficient since the size is known at compile time and the arrays can be stack-allocated or embedded directly into the binary.
**Action:** Always prefer iterating over array literals instead of `vec![...]` for statically known collections, especially in hot paths or initialization loops.

## 2024-05-25 - Avoid double lookup and lock contention in Context lookup
**Learning:** `HashMap::get(name)` followed by `.unwrap()` inside a Mutex lock not only does double lookup but keeps the lock longer than necessary when executing an inner function or cloning a value.
**Action:** Use a single lookup, clone the value or arc into a local variable (`Option<ContextValue>`), and release the lock immediately before execution.

## 2024-05-26 - Optimize decimal conversion in Value
**Learning:** `rust_decimal::Decimal` allows efficient and direct conversion to basic types like `i64` and `f64` via `to_i64` and `to_f64` using the `rust_decimal::prelude::ToPrimitive` trait. Converting to string and then parsing `val.to_string().parse()` is an anti-pattern as it incurs heap allocation overhead, which is bad for performance. When doing integer conversions from `Decimal`, it's critical to check `val.scale() == 0` first to maintain behavioral parity with string parsing (which would reject floats).
**Action:** Always favor direct conversion traits like `rust_decimal::prelude::ToPrimitive` methods over stringification when converting `Decimal` values to native numeric types. Keep edge cases like `scale()` properties in mind when changing conversion methods.
