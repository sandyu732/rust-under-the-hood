# Module 2 — Ownership, borrowing, slices

**When:** Week 2

This is the feature. Compile-time rules replace the garbage collector, and in return you get no GC pauses, no use-after-free, no double free, no iterator invalidation and no data races. Spend the most time here; everything later builds on it.

## Core concepts

- Each value has exactly one **owner**. When the owner goes out of scope, the value is **dropped**: RAII on every value.
- Assignment and pass-by-value **move** ownership. Types that implement `Copy` (`i32`, `bool`, `char`, `f64`…) are copied instead. `.clone()` is an explicit, visible deep copy.
- **Borrowing:** any number of `&T` *or* exactly one `&mut T` at a time, never both.
- **Slices** (`&str`, `&[T]`) are borrowed views. Prefer them as parameters over `&String` / `&Vec<T>`.
- Signature rule of thumb: read → `&T`; modify in place → `&mut T`; keep or transform → `T`.

## Labs

| Lab | Crate | Done when |
|---|---|---|
| **2.1** Break the borrow checker | `lab-2-1-borrow-checker` | `NOTES.md` has five entries (built with `--features broken`) and the fixed versions pass. **The most important lab in this path.** |
| **2.2** Zero-copy word tools | `lab-2-2-word-tools` | The tests pass, including the check that returned slices point into the input. |
| **2.3** Three signatures, one job | `lab-2-3-signatures` | The tests pass, including the pointer check that the owned version reuses its buffer. |
| **2.4** Drop tracer | `lab-2-4-drop-tracer` | Your *predicted* drop order matches reality. Predict first, then run. |

```bash
cargo test -p lab-2-1-borrow-checker
cargo test -p lab-2-2-word-tools
cargo test -p lab-2-3-signatures
cargo test -p lab-2-4-drop-tracer
./scripts/progress.sh m02   # this module's scoreboard
```

## Rustlings (parallel track)

Do these sections after this module: `05_vecs`, `06_move_semantics`, `09_strings`.

## Checkpoint

Answer the questions in [CHECKPOINT.md](CHECKPOINT.md) before moving on.
