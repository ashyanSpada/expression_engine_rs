## 2024-05-24 - Avoid vec! for constant collections in initialization loops
**Learning:** Initializing maps/operator managers by iterating over `vec![...]` causes unnecessary heap allocations. Using array literals `[...]` is significantly more efficient since the size is known at compile time and the arrays can be stack-allocated or embedded directly into the binary.
**Action:** Always prefer iterating over array literals instead of `vec![...]` for statically known collections, especially in hot paths or initialization loops.
