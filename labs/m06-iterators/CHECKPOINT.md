# Module 6 checkpoint

Write your answers in your own words before checking the `solutions` branch.

### 1. After `let w: Vec<_> = v.iter().map(f).collect();` can you still use `v`? What about after `v.into_iter()`?

<!-- ANSWER-START -->After `iter()`, yes: it only borrowed `v`. After `into_iter()`, no: `v` was moved into the iterator and consumed.<!-- ANSWER-END -->

### 2. Why is an iterator chain usually as fast as a loop?

<!-- ANSWER-START -->Every adapter is a small generic struct. Monomorphization and inlining fuse the whole chain into a single loop, and iterating a slice also removes bounds checks.<!-- ANSWER-END -->

### 3. Which closure trait does `|| counter += 1` implement, and why?

<!-- ANSWER-START -->`FnMut`, because it mutates captured state. It's not `Fn`. Calling it requires `&mut` access to the closure.<!-- ANSWER-END -->

### 4. When do you need `move` on a closure?

<!-- ANSWER-START -->When the closure must outlive the current stack frame: returning it, spawning a thread, or storing it. `move` makes it own its captures instead of borrowing them.<!-- ANSWER-END -->
