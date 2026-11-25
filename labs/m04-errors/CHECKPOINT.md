# Module 4 checkpoint

Write your answers in your own words before checking the `solutions` branch.

### 1. What happens if you use `?` on an `Option` inside a function that returns `Result`?

<!-- ANSWER-START -->It doesn't compile. `?` needs the function's return type to match, so convert first: `opt.ok_or(MyError::Missing)?`.<!-- ANSWER-END -->

### 2. When is `unwrap()` acceptable?

<!-- ANSWER-START -->In tests and throwaway prototypes, and where an invariant guarantees success (for example, a regex literal that's covered by tests). Prefer `expect("why this can't fail")` so the panic message documents the assumption.<!-- ANSWER-END -->

### 3. How does `collect::<Result<Vec<_>, _>>()` behave?

<!-- ANSWER-START -->It stops at the first `Err` and returns it; otherwise it returns `Ok` with every value. That's fail-fast validation in one line.<!-- ANSWER-END -->

### 4. Why do libraries prefer `thiserror` and applications prefer `anyhow`?

<!-- ANSWER-START -->Library callers need to match on specific failure kinds, so libraries need a typed enum. Applications mostly report errors, so one opaque type with a context chain is simpler.<!-- ANSWER-END -->
