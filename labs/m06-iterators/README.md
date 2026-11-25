# Module 6 — Collections, iterators, closures

**When:** Week 4

Iterator chains read like Java Streams or Python comprehensions, but they compile to the same machine code as a hand-written loop. Idiomatic Rust rarely indexes into arrays.

## Core concepts

- `iter()` → `&T`, `iter_mut()` → `&mut T`, `into_iter()` → `T`. These mirror the three ownership modes.
- Adapters are lazy (`map`, `filter`, `zip`, `flat_map`, `take_while`, `windows`…); consumers drive them (`collect`, `sum`, `fold`, `find`, `any`).
- `collect()` builds whatever the target type asks for: `Vec`, `HashMap`, `String`, `Result<Vec<_>, _>`.
- HashMap entry API: `*map.entry(k).or_insert(0) += 1`.
- Closures are `Fn` / `FnMut` / `FnOnce` depending on how they capture. `move` forces ownership of what they capture.

## Labs

| Lab | Crate | Done when |
|---|---|---|
| **6.1** Word frequency | `lab-6-1-word-freq` | The tests pass with no index loops. Then run it on a Gutenberg book in `--release`. |
| **6.2** Custom iterators | `lab-6-2-custom-iterators` | The tests pass, and Fibonacci stops before overflowing. |
| **6.3** Log analyzer | `lab-6-3-log-analyzer` | The tests pass, and `analyze` on 100k generated lines takes under a second in `--release`. |
| **6.4** Zero-cost proof | `lab-6-4-zero-cost` | The tests pass, and `cargo run --release -p lab-6-4-zero-cost` shows the three versions with similar timings. |
| **6.5** Closure kinds | `lab-6-5-closures` | The tests pass, and you can explain why `FnOnce` is the most permissive bound to accept. |

```bash
cargo test -p lab-6-1-word-freq
cargo test -p lab-6-2-custom-iterators
cargo test -p lab-6-3-log-analyzer
cargo test -p lab-6-4-zero-cost
cargo test -p lab-6-5-closures
./scripts/progress.sh m06   # this module's scoreboard
```

## Rustlings (parallel track)

Do these sections after this module: `11_hashmaps`, `18_iterators`.

## Checkpoint

Answer the questions in [CHECKPOINT.md](CHECKPOINT.md) before moving on.
