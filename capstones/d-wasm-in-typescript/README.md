# Capstone D: Rust in your TypeScript app via WebAssembly (spec)

Compile a CPU-heavy function to WebAssembly and call it from a TypeScript frontend.

## Setup

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
cargo new --lib capstones/d-wasm-in-typescript/fast-core
```

In `fast-core/Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
```

Then add `"capstones/d-wasm-in-typescript/fast-core"` to the workspace `members`.

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn levenshtein(a: &str, b: &str) -> u32 {
    // your implementation
    0
}
```

```bash
cd capstones/d-wasm-in-typescript/fast-core
wasm-pack build --target web --release     # → pkg/ with .wasm + generated TypeScript types
```

## Requirements

1. Pick a function worth speeding up: Levenshtein distance over many strings, an image filter (grayscale/blur on `Uint8ClampedArray`), Markdown → HTML (wrap `pulldown-cmark`), or a diff algorithm.
2. Write the same function in plain TypeScript.
3. A small Vite + TS page that runs both on the same input and shows the timings.
4. Pass data efficiently: `&[u8]` / `Vec<u8>` map to `Uint8Array`. Avoid serialising large inputs to JSON.
5. Unit-test the Rust core with ordinary `cargo test` (the `rlib` crate type allows that).

## Done when

- [ ] `npm run dev` shows both implementations producing identical output
- [ ] A timing table for 3 input sizes is in the README (expect Wasm to win on the large inputs and lose on the tiny ones, because of call overhead)
- [ ] The `.wasm` size is reported (try `opt-level = "z"`, `lto = true` and `wasm-opt`)
