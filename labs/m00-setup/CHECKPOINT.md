# Module 0 checkpoint

Write your answers in your own words before checking the `solutions` branch.

### 1. What's the difference between `cargo check` and `cargo build`?

<!-- ANSWER-START -->`check` runs the full type and borrow check but skips code generation, so it's several times faster and produces no binary. Use it constantly; use `build`/`run` when you need to execute.<!-- ANSWER-END -->

### 2. Where do the docs for a dependency come from, and how do you read them offline?

<!-- ANSWER-START -->They're generated from the crate's `///` doc comments. `cargo doc --open` builds docs for your crate and every dependency locally; docs.rs hosts the same for every published version.<!-- ANSWER-END -->

### 3. What does `Cargo.lock` do, and should it be committed?

<!-- ANSWER-START -->It records the exact resolved version of every dependency so builds are reproducible. Commit it for applications and workspaces (like this repo); libraries usually commit it too nowadays, though consumers ignore it.<!-- ANSWER-END -->
