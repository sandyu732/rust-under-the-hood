# Module 3 checkpoint

Write your answers in your own words before checking the `solutions` branch.

### 1. How is a Rust enum different from a Java enum?

<!-- ANSWER-START -->Each Rust variant can carry its own differently shaped data (`Card { last4 }`, `Crypto(String)`), and `match` must handle every variant. It's closer to a sealed interface with record implementations plus exhaustive switch.<!-- ANSWER-END -->

### 2. When should a method take `self` instead of `&self`?

<!-- ANSWER-START -->When it consumes or transforms the value into something else. State transitions are the classic case: `fn ship(self) -> Result<Order, _>` makes the old state unusable afterwards.<!-- ANSWER-END -->

### 3. What does the newtype pattern cost at runtime?

<!-- ANSWER-START -->Nothing. `struct UserId(u64)` has exactly the same size and layout as `u64`; the distinction exists only at compile time.<!-- ANSWER-END -->

### 4. Why does adding a variant to an enum help with refactoring?

<!-- ANSWER-START -->Every non-exhaustive `match` becomes a compile error pointing at the exact spot to update, so no call site gets missed.<!-- ANSWER-END -->
