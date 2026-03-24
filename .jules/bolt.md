## 2024-05-24 - Avoid vec! for constant collections in initialization loops
**Learning:** Initializing maps/operator managers by iterating over `vec![...]` causes unnecessary heap allocations. Using array literals `[...]` is significantly more efficient since the size is known at compile time and the arrays can be stack-allocated or embedded directly into the binary.
**Action:** Always prefer iterating over array literals instead of `vec![...]` for statically known collections, especially in hot paths or initialization loops.

## 2024-05-25 - Avoid double lookup and lock contention in Context lookup
**Learning:** `HashMap::get(name)` followed by `.unwrap()` inside a Mutex lock not only does double lookup but keeps the lock longer than necessary when executing an inner function or cloning a value.
**Action:** Use a single lookup, clone the value or arc into a local variable (`Option<ContextValue>`), and release the lock immediately before execution.
