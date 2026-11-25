# Module 0 — Toolchain setup

**When:** 1–2 hours

Cargo is the build tool, package manager, test runner, doc generator and benchmark runner in one. Think of the `go` CLI rather than Maven + JUnit + Javadoc. Work through [GETTING_STARTED.md](../../GETTING_STARTED.md) sections 1–5 first.

## Core concepts

- `rustup` manages toolchains; `cargo` drives everything else.
- **Crate** = compilation unit, **package** = a `Cargo.toml`, **workspace** = packages sharing one lock file and `target/`.
- `cargo check` is your fast feedback loop; `cargo build --release` is for speed measurements and shipping.

## Labs

| Lab | Crate | Done when |
|---|---|---|
| **0.1** Workspace member | *(you create it)* `lab-0-1-hello` | `cargo new labs/m00-setup/lab-0-1-hello`, then `cargo run -p lab-0-1-hello` works from the repo root. It joins the workspace automatically through the `labs/*/lab-*` glob. |
| **0.2** Break it on purpose | `lab-0-1-hello` | In `main`, write `let x: i32 = "five";`. Read the whole error, then run `rustc --explain E0308`. You can point to the line of the error message that tells you the fix. |
| **0.3** Rustlings warm-up | — | `cargo install rustlings && rustlings init` (outside this repo). The `00_intro`, `01_variables` and `02_functions` sections are done. |

```bash
cargo test -p lab-0-1-hello
./scripts/progress.sh m00   # this module's scoreboard
```

## Rustlings (parallel track)

Do these sections after this module: `00_intro`, `01_variables`, `02_functions`.

## Checkpoint

Answer the questions in [CHECKPOINT.md](CHECKPOINT.md) before moving on.
