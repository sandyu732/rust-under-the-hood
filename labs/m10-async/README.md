# Module 10 — Async Rust with Tokio

**When:** Week 7

For I/O-bound services (APIs, proxies, crawlers), async lets one thread juggle thousands of connections. Unlike Go, the runtime isn't built in: you choose one, and Tokio is the de facto standard.

## Core concepts

- `async fn` returns a `Future`. Futures are **lazy** (unlike a JS `Promise`): nothing runs until something `.await`s or spawns them.
- `tokio::spawn` is the goroutine analogue. Spawned futures must be `Send + 'static`.
- `join!` waits for all, `select!` takes whichever finishes first, `timeout` puts a deadline on anything. A `Semaphore` bounds concurrency.
- Never block inside async code. Use `tokio::time::sleep`, and `spawn_blocking` for CPU-heavy or blocking work.
- The usual service stack: `tokio` + `axum` + `serde` + `sqlx` + `tracing`.

## Labs

| Lab | Crate | Done when |
|---|---|---|
| **10.1** Concurrent URL checker | `lab-10-1-url-checker` | The tests pass (they use a fake fetcher on Tokio's paused clock). Then run it on a real list of URLs. |
| **10.2** TCP chat server | `lab-10-2-chat-server` | The tests pass. Try it with `nc localhost 8080` in three terminals. |
| **10.3** REST API with axum | `lab-10-3-axum-todos` | The tests pass. `cargo run -p lab-10-3-axum-todos`, then try it with `curl`. |
| **10.4** Add a database | `lab-10-4-sqlx-todos` | The same test suite passes against SQLite. |

```bash
cargo test -p lab-10-1-url-checker
cargo test -p lab-10-2-chat-server
cargo test -p lab-10-3-axum-todos
cargo test -p lab-10-4-sqlx-todos
./scripts/progress.sh m10   # this module's scoreboard
```

## Checkpoint

Answer the questions in [CHECKPOINT.md](CHECKPOINT.md) before moving on.
