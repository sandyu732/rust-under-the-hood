# Module 4 — Error handling

**When:** Week 3

There are no exceptions. Failure is part of the function's type, so callers can't forget it. It's Go's explicitness with far less boilerplate.

## Core concepts

- `Option<T>` (`Some`/`None`) and `Result<T, E>` (`Ok`/`Err`) are ordinary enums.
- `?` returns early with the error, converting it through `From`. It replaces `if err != nil { return err }`.
- Combinators: `map`, `map_err`, `and_then`, `ok_or`, `unwrap_or_else`, `unwrap_or_default`.
- `panic!` is for bugs, `Result` is for expected failures. `expect("reason")` beats `unwrap()`.
- Ecosystem convention: `thiserror` for libraries (typed errors callers can match on), `anyhow` for applications (one error type, with context).

## Labs

| Lab | Crate | Done when |
|---|---|---|
| **4.1** Refactor away panics | `lab-4-1-cli-errors` | The tests pass, and there is no `.unwrap()` and no `match` on errors in `src/lib.rs` (a test checks this). |
| **4.2** Config loader, by hand | `lab-4-2-config-loader` | The tests pass, with `Display`, `Error::source` and `From<io::Error>` written yourself. |
| **4.3** thiserror + anyhow | `lab-4-3-thiserror-anyhow` | The tests pass. Run the binary on a missing file and read the `Caused by:` chain. |
| **4.4** CSV stats | `lab-4-4-csv-stats` | The tests pass in both strict and lenient modes. |

```bash
cargo test -p lab-4-1-cli-errors
cargo test -p lab-4-2-config-loader
cargo test -p lab-4-3-thiserror-anyhow
cargo test -p lab-4-4-csv-stats
./scripts/progress.sh m04   # this module's scoreboard
```

## Rustlings (parallel track)

Do these sections after this module: `12_options`, `13_error_handling`, `23_conversions`.

## Checkpoint

Answer the questions in [CHECKPOINT.md](CHECKPOINT.md) before moving on.
