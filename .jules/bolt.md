## 2024-05-24 - Avoid vec! for constant collections in initialization loops
**Learning:** Initializing maps/operator managers by iterating over `vec![...]` causes unnecessary heap allocations. Using array literals `[...]` is significantly more efficient since the size is known at compile time and the arrays can be stack-allocated or embedded directly into the binary.
**Action:** Always prefer iterating over array literals instead of `vec![...]` for statically known collections, especially in hot paths or initialization loops.

## 2024-05-24 - [Format Display Optimizations]
**Learning:** For `std::fmt::Display` implementations involving collections (like Lists or Maps), building an intermediate `String` via `format!()` and `push_str()` creates unnecessary heap allocations and redundant cloning. Writing directly to the formatter using `write!(f, ...)` avoids intermediate `String` allocations entirely.
**Action:** Always write directly to the `std::fmt::Formatter` inside `fmt` methods rather than creating an intermediate string representation, particularly when dealing with container-like structures.
