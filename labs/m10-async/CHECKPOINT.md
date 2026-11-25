# Module 10 checkpoint

Write your answers in your own words before checking the `solutions` branch.

### 1. Why does `tokio::spawn` require `Send`?

<!-- ANSWER-START -->Tokio's default runtime is multi-threaded and may move a task to another worker thread at any `.await`, so everything the task holds across an await must be safe to send.<!-- ANSWER-END -->

### 2. What happens to the server if one handler calls `std::thread::sleep(5s)`?

<!-- ANSWER-START -->It blocks that worker thread for 5 seconds. Every other task scheduled on that thread stalls, and on a single-threaded runtime the whole server freezes. Use `tokio::time::sleep` or `spawn_blocking` instead.<!-- ANSWER-END -->

### 3. Why are Rust futures lazy, and what bug does that cause for JS developers?

<!-- ANSWER-START -->A future is just a state machine; it makes progress only when polled. Calling an `async fn` without `.await`ing or spawning the result does nothing at all (the compiler warns about it).<!-- ANSWER-END -->

### 4. In the chat server, why subscribe to the broadcast channel *before* sending the welcome line?

<!-- ANSWER-START -->So no message can arrive between "the client thinks it has joined" and "the server is listening for it". Subscribing first closes that race, and the tests rely on it.<!-- ANSWER-END -->
