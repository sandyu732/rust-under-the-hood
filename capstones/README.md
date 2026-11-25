# Capstone projects

Pick one or two. Each pulls together most modules and gives you something worth showing. For every capstone, write a README, keep clippy clean, and add a short "what I'd do differently" note.

| Project | Type | What you build | Modules | Done when |
|---|---|---|---|---|
| [**A. rgrep**](a-rgrep) | Tests included | A ripgrep-lite: regex search, `.gitignore`-aware walk, binary detection, parallel search, coloured output | 2, 4, 5, 6, 9 | `cargo test -p capstone-a-rgrep` passes; benchmarked against `grep -r` with `hyperfine`, within 2× of it |
| [**B. mini-Redis**](b-mini-redis) | Tests included | An async TCP key-value server: `GET`/`SET EX`/`DEL`/`INCR`, append-only-file persistence | 3, 4, 8, 9, 10 | `cargo test -p capstone-b-mini-redis` passes; survives 1,000 concurrent clients from a load script |
| [**C. Production service**](c-production-service) | Spec only | Lab 10.4 grown up: Postgres, config, tracing, graceful shutdown, Docker, CI | 4, 5, 10 | Image < 30 MB, CI green, p99 latency measured |
| [**D. Rust in your TypeScript app**](d-wasm-in-typescript) | Spec only | A CPU-heavy function compiled to WebAssembly and called from TS | 1, 2, 5, 6 | Benchmarked against a pure-TS version |
| [**E. Python extension**](e-python-extension) | Spec only | A slow Python function rewritten in Rust with PyO3 + maturin | 1, 2, 4, 6 | An installable wheel, benchmarked against the Python original |

D and E are the easiest way to bring Rust into your current day job without a rewrite.

```bash
cargo test -p capstone-a-rgrep
cargo run --release -p capstone-a-rgrep -- 'fn \w+' labs -i

cargo test -p capstone-b-mini-redis
cargo run -p capstone-b-mini-redis -- --aof /tmp/data.aof   # then: nc localhost 6380
```

**Packaging practice:** once A works, follow [GETTING_STARTED.md §11](../GETTING_STARTED.md#11-packaging-and-distribution). `cargo install --path capstones/a-rgrep`, build a static musl binary, and put B in a multi-stage Docker image.
