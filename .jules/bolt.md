## 2024-03-12 - [Context Map Lookups]
**Learning:** Found an edge case where map access and function execution was holding a Mutex lock which leads to high lock contention, especially since Context instances store closures that might be arbitrarily expensive. Double map lookups in `.value()` (`is_none()` then `unwrap()`) combined with lock-holding was a bottleneck for expression execution.
**Action:** Always fetch the target value into an owned variable outside the lock block (using scope bounds or `cloned()`) in high-frequency map lookups. Never execute user closures inside a lock guard.
