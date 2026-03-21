
## 2024-03-21 - Optimize Context state access and lock contention
**Learning:** Holding a Mutex lock while invoking function pointers in Context can lead to lock contention and blocking threads when context variables are computed by long-running functions. Moreover, multiple map lookups and excessive cloning create micro-bottlenecks.
**Action:** Extract cloned value before invoking the inner function inside the Context value mapping, drop the MutexGuard as early as possible and optimize Hashmap lookups with pattern matching to avoid doing redundant copies of values.
