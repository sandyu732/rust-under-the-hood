# Module 9 checkpoint

Write your answers in your own words before checking the `solutions` branch.

### 1. Why does `thread::spawn` require `'static` while `scope.spawn` doesn't?

<!-- ANSWER-START -->A spawned thread may outlive the function that spawned it, so it can't borrow that function's locals. A scope guarantees every thread is joined before the scope returns, so borrows are safe.<!-- ANSWER-END -->

### 2. Lab 9.1(a) in Go: what happens?

<!-- ANSWER-START -->It compiles and races: the result is usually wrong and differs between runs. Only `go run -race` flags it, and only if the race happens during that run.<!-- ANSWER-END -->

### 3. Why is `Arc<Mutex<T>>` both `Send` and `Sync` when `T` is only `Send`?

<!-- ANSWER-START -->`Mutex` serialises access, which makes `Mutex<T>` `Sync` whenever `T: Send`. Then `Arc<Mutex<T>>` is `Send + Sync` because its contents are.<!-- ANSWER-END -->

### 4. How does a channel-based pipeline shut down without a stop flag?

<!-- ANSWER-START -->Each stage owns its `Sender`. When a stage finishes, its sender drops; once every sender is gone, the receiver's iterator returns `None`, so the next stage's loop ends and it drops its own sender in turn.<!-- ANSWER-END -->
