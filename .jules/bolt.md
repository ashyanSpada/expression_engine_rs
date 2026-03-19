
## 2024-05-28 - [Reduced Context Lock Contention and Double Lookups]
**Learning:** Holding a lock on global/shared state while executing custom closures in an expression engine can lead to severe lock contention. Furthermore, `HashMap` lookups should always leverage `Option::cloned()` where applicable to avoid redundant matching and cloning.
**Action:** Always extract the owned value and drop the lock prior to performing long-running or unknown execution times in functions like `Context::value`. Use `.cloned()` and pattern matching directly.
