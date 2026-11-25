# Module 1 checkpoint

Write your answers in your own words before checking the `solutions` branch.

### 1. Why is `char` 4 bytes, while a `&str` holding one ASCII letter points to only 1 byte of data?

<!-- ANSWER-START -->`char` must hold any Unicode scalar value (up to U+10FFFF), so it's a fixed 4 bytes. Strings are UTF-8, a variable-width encoding where ASCII takes 1 byte. A `&str` itself is 16 bytes (pointer + length).<!-- ANSWER-END -->

### 2. What does `let x = { 5; };` assign to `x`?

<!-- ANSWER-START -->`()`, the unit value. The trailing `;` turns `5` into a statement, so the block has no final expression.<!-- ANSWER-END -->

### 3. `255u8 + 1` panics in a test but not in a `--release` binary. Why, and what should production code do?

<!-- ANSWER-START -->Debug builds enable overflow checks; release builds wrap by default for speed. Production code that can overflow should choose explicitly: `checked_add` (and handle `None`), `saturating_add`, or `wrapping_add`.<!-- ANSWER-END -->

### 4. Why does `Option<bool>` take 1 byte but `Option<u8>` takes 2?

<!-- ANSWER-START -->Niche optimisation: `bool` only uses the bit patterns 0 and 1, so `None` can be stored as 2. Every `u8` value is valid, so `Option<u8>` needs a separate tag byte.<!-- ANSWER-END -->
