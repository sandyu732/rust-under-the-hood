# Module 9 — Fearless concurrency

**When:** Week 6

In Java and Go, a data race is a runtime bug you find with a race detector, if you're lucky. In Rust it's a compile error. The ownership rules from Module 2 plus two marker traits are the whole mechanism.

## Core concepts

- `thread::spawn(move || …)` needs `'static + Send` closures. `thread::scope` lets threads borrow locals safely.
- `Send`: safe to move to another thread. `Sync`: safe to share `&T` between threads. The compiler implements both automatically.
- A `Mutex<T>` *owns* its data, so you can't touch the data without locking. The guard unlocks when it's dropped.
- Channels: `mpsc::channel` (unbounded), `sync_channel(n)` (bounded, with back-pressure). Dropping every `Sender` ends the receiving loop.
- `rayon`: change `.iter()` to `.par_iter()` for CPU-bound data parallelism.

## Labs

| Lab | Crate | Done when |
|---|---|---|
| **9.1** Let the compiler stop you | `lab-9-1-shared-counter` | `NOTES.md` is filled in (built with `--features broken`) and all three counters plus `parallel_sum` pass. |
| **9.2** Parallel file hasher | `lab-9-2-file-hasher` | The pool and rayon versions agree. Compare their speed with `cargo run --release -p lab-9-2-file-hasher -- ~/some/dir`. |
| **9.3** Bounded pipeline | `lab-9-3-pipeline` | The tests pass, including the 100k-line shutdown test. |
| **9.4** Concurrent cache | `lab-9-4-concurrent-cache` | The tests pass, including the deadlock test (no lock held while computing). |

```bash
cargo test -p lab-9-1-shared-counter
cargo test -p lab-9-2-file-hasher
cargo test -p lab-9-3-pipeline
cargo test -p lab-9-4-concurrent-cache
./scripts/progress.sh m09   # this module's scoreboard
```

## Rustlings (parallel track)

Do these sections after this module: `20_threads`.

## Checkpoint

Answer the questions in [CHECKPOINT.md](CHECKPOINT.md) before moving on.
