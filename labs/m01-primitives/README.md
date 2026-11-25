# Module 1 — Primitive types, variables, expressions

**When:** Week 1

Rust's primitives have explicit widths, are unboxed and live on the stack. There's no `Integer` vs `int` split like Java, no arbitrary-precision surprise like Python, and no "every number is a float" like JavaScript.

## Core concepts

- Integers `i8`…`i128`, `u8`…`u128`, `isize`/`usize` (pointer-width, used for indexing). Floats `f32`/`f64`. `bool` (no truthiness). `char` is a 4-byte Unicode scalar value.
- Immutable by default; `let mut` opts in. Shadowing (`let x = x.trim();`) rebinds a name, even with a new type.
- No implicit numeric conversion: `as` is lossy, `TryFrom`/`try_into()` is checked.
- Overflow panics in debug builds and wraps in release. Say what you mean: `checked_*`, `wrapping_*`, `saturating_*`, `overflowing_*`.
- `if`, `match`, `loop` and blocks are expressions. The last expression without a `;` is the value.

## Labs

| Lab | Crate | Done when |
|---|---|---|
| **1.1** Type explorer | `lab-1-1-type-explorer` | Every size prediction is right, and you can explain why `Option<Box<i32>>` is the same size as `Box<i32>`. `cargo run -p lab-1-1-type-explorer` prints the table. |
| **1.2** Overflow lab | `lab-1-2-overflow` | The tests pass, and you've run `cargo run -p lab-1-2-overflow -- 250 10` with and without `--release` and noted the difference. |
| **1.3** Expression-only code | `lab-1-3-expressions` | The tests pass, with no `return` keyword and no `mut` outside `collatz_steps`. |
| **1.4** Base converter CLI | `lab-1-4-base-converter` | The tests pass, and `cargo run -p lab-1-4-base-converter -- abc hex` prints a friendly error and exits with code 1. |

```bash
cargo test -p lab-1-1-type-explorer
cargo test -p lab-1-2-overflow
cargo test -p lab-1-3-expressions
cargo test -p lab-1-4-base-converter
./scripts/progress.sh m01   # this module's scoreboard
```

## Rustlings (parallel track)

Do these sections after this module: `03_if`, `04_primitive_types`.

## Checkpoint

Answer the questions in [CHECKPOINT.md](CHECKPOINT.md) before moving on.
