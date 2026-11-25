# Module 2 checkpoint

Write your answers in your own words before checking the `solutions` branch.

### 1. Why can't Rust have Java's `ConcurrentModificationException`?

<!-- ANSWER-START -->Iterating holds a shared borrow `&v`; mutating needs `&mut v`. The borrow rules forbid both at once, so the bug is a compile error instead of a runtime exception.<!-- ANSWER-END -->

### 2. When do you take `String` and when `&str` as a parameter?

<!-- ANSWER-START -->Take `&str` when you only read it (it accepts `&String`, literals and slices). Take `String` when the function needs to own it: store it in a struct, send it to another thread, or return it modified without reallocating.<!-- ANSWER-END -->

### 3. What does moving a `Vec` of a million elements copy?

<!-- ANSWER-START -->Only the 24-byte header (pointer, capacity, length). The heap buffer stays where it is; ownership of it changes hands.<!-- ANSWER-END -->

### 4. What's the drop order of locals at the end of a scope?

<!-- ANSWER-START -->Reverse order of declaration. A `Vec` drops its elements front to back. A value moved into a function is dropped at the end of that function, unless it's returned or moved on again.<!-- ANSWER-END -->
