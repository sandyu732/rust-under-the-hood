# Module 7 checkpoint

Write your answers in your own words before checking the `solutions` branch.

### 1. Why does `fn first_line(&self) -> &str` need no annotation?

<!-- ANSWER-START -->Elision rule: with a `&self` receiver, every elided output lifetime is tied to `self`.<!-- ANSWER-END -->

### 2. What's wrong with adding `'static` to silence lifetime errors?

<!-- ANSWER-START -->It's a much stronger promise than you need. It forces callers to pass only data that lives for the whole program (literals or leaked memory), and it usually just moves the error to the call site.<!-- ANSWER-END -->

### 3. `Highlighter::find_all` returns `Vec<&'t str>`, not `Vec<&str>`. What's the difference for callers?

<!-- ANSWER-START -->With `'t`, the results borrow the original text, so they remain valid after the highlighter is dropped. With elision they'd borrow `&self`, and the highlighter would have to outlive them.<!-- ANSWER-END -->

### 4. When is `Cow<'a, str>` the right return type?

<!-- ANSWER-START -->When a function usually returns its input unchanged but sometimes needs to modify it. It borrows in the common case and allocates only when needed.<!-- ANSWER-END -->
