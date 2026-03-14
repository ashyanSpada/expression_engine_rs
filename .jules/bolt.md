## 2024-03-14 - Optimize Context value retrieval and lock duration
**Learning:** `Context::value` performed two `HashMap::get` lookups to retrieve a variable/function from the shared Context state, and evaluated dynamic inner functions while holding the `MutexGuard` on the entire variables map, causing lock contention and redundant hashmap lookups. This architecture specifically caused overhead during expression execution.
**Action:** Use a single `.get().cloned()` to retrieve the `ContextValue` and drop the mutex early, before the `match` executing any dynamic functions.
