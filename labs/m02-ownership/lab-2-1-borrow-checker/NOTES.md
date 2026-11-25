# Lab 2.1 notes

Fill in one row per broken function after reading the compiler error.

| Fn | Error code | What the compiler is protecting you from (one sentence) | How you fixed it |
|---|---|---|---|
| (a) use after move | <!-- ANSWER-START -->E0382<!-- ANSWER-END --> | <!-- ANSWER-START -->`let t = s` moved the String into `t`, so `s` no longer owns anything; using it would read freed/duplicated memory.<!-- ANSWER-END --> | <!-- ANSWER-START -->Borrow with `&s` (or `clone()` if two owners are really needed).<!-- ANSWER-END --> |
| (b) two `&mut` | <!-- ANSWER-START -->E0499<!-- ANSWER-END --> | <!-- ANSWER-START -->Two live mutable references could interleave writes — the root cause of data races.<!-- ANSWER-END --> | <!-- ANSWER-START -->End the first borrow (scope it) before creating the second.<!-- ANSWER-END --> |
| (c) push while borrowed | <!-- ANSWER-START -->E0502<!-- ANSWER-END --> | <!-- ANSWER-START -->`push` may reallocate the buffer, leaving `first` pointing at freed memory (use-after-free).<!-- ANSWER-END --> | <!-- ANSWER-START -->Copy the value out (`let first = v[0]`) before mutating.<!-- ANSWER-END --> |
| (d) dangling | <!-- ANSWER-START -->E0106<!-- ANSWER-END --> | <!-- ANSWER-START -->`s` is dropped when the function returns; a reference to it would dangle.<!-- ANSWER-END --> | <!-- ANSWER-START -->Return the owned `String`.<!-- ANSWER-END --> |
| (e) mutate while iterating | <!-- ANSWER-START -->E0502<!-- ANSWER-END --> | <!-- ANSWER-START -->The iterator holds `&v`; pushing could reallocate under it (iterator invalidation / Java's ConcurrentModificationException).<!-- ANSWER-END --> | <!-- ANSWER-START -->Collect the new values first, then `extend`.<!-- ANSWER-END --> |

**Bonus:** write (c) in Java with an `ArrayList` and an iterator, and note what happens at runtime.
