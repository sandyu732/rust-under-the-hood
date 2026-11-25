# Lab 7.1 notes

1. **E0106 in `broken::longest`** — what is the compiler asking for, in plain words?
   <!-- ANSWER-START -->With two reference inputs, elision can't decide which one the output borrows from, so it asks us to name the relationship: `<'a>(a: &'a str, b: &'a str) -> &'a str` says "the result lives no longer than the shorter of the two".<!-- ANSWER-END -->

2. **E0597 in `broken::caller`** — why is this rejected even though at runtime `longest` would return `a`?
   <!-- ANSWER-START -->The borrow checker reasons about signatures, not runtime values. The signature says the result may borrow from `b`, and `b` dies at the end of the inner block, so using `result` afterwards could be a dangling reference.<!-- ANSWER-END -->

3. **Fix for `caller` without changing `longest`:**
   <!-- ANSWER-START -->Move `let b` to the outer scope (so it lives as long as `result`), or use `prefix_before`-style signatures that only tie the output to `a`, or return an owned `String`.<!-- ANSWER-END -->
