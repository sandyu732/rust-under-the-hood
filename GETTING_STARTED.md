# Getting Started with Rust and Cargo

A practical tour of the toolchain for developers coming from Python, TypeScript, Java or Go. Work through it once (about 60–90 minutes) before Module 1; come back to it as a reference.

- [1. Install the toolchain](#1-install-the-toolchain)
- [2. Set up your editor](#2-set-up-your-editor)
- [3. Your first project in 10 minutes](#3-your-first-project-in-10-minutes)
- [4. Anatomy of a Cargo project](#4-anatomy-of-a-cargo-project)
- [5. The everyday commands](#5-the-everyday-commands)
- [6. Dependencies and crates.io](#6-dependencies-and-cratesio)
- [7. Testing](#7-testing)
- [8. Build profiles: debug vs release](#8-build-profiles-debug-vs-release)
- [9. Workspaces (how this repo is organised)](#9-workspaces-how-this-repo-is-organised)
- [10. Features and conditional compilation](#10-features-and-conditional-compilation)
- [11. Packaging and distribution](#11-packaging-and-distribution)
- [12. Useful extra tools](#12-useful-extra-tools)
- [13. Translation table from your languages](#13-translation-table-from-your-languages)

---

## 1. Install the toolchain

Rust is installed and updated with **rustup** (think `nvm` / `pyenv` / SDKMAN, but official).

**macOS / Linux**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# restart your shell, or:
source "$HOME/.cargo/env"
```

**Windows:** download and run `rustup-init.exe` from <https://rustup.rs>. It will ask you to install the Visual Studio C++ Build Tools (the linker) if they are missing — accept.

**Verify**

```bash
rustc --version      # the compiler
cargo --version      # build tool + package manager
rustup --version     # the toolchain manager
```

**What you just installed**

| Tool | What it does | Closest equivalent |
|---|---|---|
| `rustup` | Installs/updates compilers, targets and components | `nvm`, `pyenv`, SDKMAN |
| `rustc` | The compiler (you rarely call it directly) | `javac`, `tsc` |
| `cargo` | Build, run, test, deps, docs, publish | `go` CLI; Maven + JUnit + Javadoc in one |
| `clippy` | Linter with ~800 lints | ESLint, SpotBugs, `go vet` |
| `rustfmt` | Formatter (one style, no debates) | `gofmt`, Prettier, Black |
| `rust-analyzer` | Language server for your editor | Pylance, tsserver, gopls |

**Keeping it current**

```bash
rustup update                    # new stable every 6 weeks; backwards compatible
rustup component add clippy rustfmt
rustup toolchain install nightly # only if you need unstable features (you won't here)
rustup doc --book                # the Rust Book, offline
rustup doc --std                 # standard library docs, offline
```

This repo contains a `rust-toolchain.toml`, so `cargo` automatically uses stable with clippy and rustfmt installed — everyone on the project gets the same setup.

## 2. Set up your editor

- **VS Code:** install *rust-analyzer* (this repo recommends it automatically via `.vscode/extensions.json`), plus *Even Better TOML* and *CodeLLDB* for debugging.
- **JetBrains:** *RustRover*, or the Rust plugin in IntelliJ IDEA.
- **Neovim / Helix / Zed:** point your LSP client at `rust-analyzer`.

Turn on **inlay hints** (types, parameter names, chained-iterator types). Seeing the inferred types is half the learning.
Set **"check on save" to run clippy**: in VS Code settings, `"rust-analyzer.check.command": "clippy"`.

## 3. Your first project in 10 minutes

Do this inside this repo. The workspace picks up any crate under `labs/*/lab-*`.

```bash
# 1. Create a binary crate
cargo new labs/m00-setup/lab-0-1-hello
cd labs/m00-setup/lab-0-1-hello

# 2. Build and run it (debug build, fast to compile)
cargo run
#    Compiling lab-0-1-hello v0.1.0
#    Hello, world!

# 3. Add a dependency from crates.io
cargo add rand          # edits Cargo.toml for you

# 4. Use it — replace src/main.rs with:
```

```rust
use rand::RngExt; // rand 0.10; older versions use `rand::Rng`

fn main() {
    let n: u8 = rand::rng().random_range(1..=6);
    println!("You rolled a {n}");
}
```

```bash
# 5. Run again, then a release (optimised) build
cargo run
cargo build --release
./../../../target/release/lab-0-1-hello   # binaries land in the workspace's target/

# 6. Lint, format, document
cargo clippy
cargo fmt
cargo doc --open        # your crate AND every dependency's docs, in the browser

# 7. Install it as a command on your PATH (~/.cargo/bin)
cargo install --path .
lab-0-1-hello
cargo uninstall lab-0-1-hello
```

> If `rand`'s API has changed by the time you read this, that's a good lesson too: run `cargo doc --open -p rand` and read the current docs.

## 4. Anatomy of a Cargo project

```text
my-crate/
├── Cargo.toml          # manifest: name, version, edition, dependencies
├── Cargo.lock          # exact resolved versions (commit it for binaries and workspaces)
├── src/
│   ├── main.rs         # binary crate root  → `cargo run`
│   ├── lib.rs          # library crate root → `use my_crate::...`
│   ├── parser.rs       # module `parser`  (declared with `mod parser;` in lib.rs)
│   └── bin/
│       └── tool.rs     # extra binary     → `cargo run --bin tool`
├── tests/              # integration tests: each file is its own test binary
├── examples/           # `cargo run --example name`
├── benches/            # `cargo bench`
└── target/             # build output (git-ignored)
```

**Vocabulary**

- **Crate**: one compilation unit, either a library or a binary. The smallest thing you can depend on.
- **Package**: a directory with a `Cargo.toml`. It can hold one library and any number of binaries.
- **Module**: a namespace inside a crate (`mod`). Files map to modules. Everything is private unless marked `pub`.
- **Workspace**: several packages that share one `Cargo.lock` and one `target/` (section 9).
- **Edition**: `2015` / `2018` / `2021` / `2024`. A per-crate language version. Crates of different editions link together fine. New code should use `2024`.

**A minimal `Cargo.toml`**

```toml
[package]
name = "my-crate"
version = "0.1.0"          # SemVer
edition = "2024"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
anyhow = "1"

[dev-dependencies]         # only for tests, examples and benches
tempfile = "3"
```

**Modules in 30 seconds**

```rust
// src/lib.rs
pub mod parser;            // looks for src/parser.rs (or src/parser/mod.rs)
mod internal;              // private to this crate

pub use parser::parse;     // re-export: callers write my_crate::parse

// src/parser.rs
pub fn parse(s: &str) -> Vec<&str> { s.split(',').collect() }
pub(crate) fn helper() {}  // visible inside this crate only
```

## 5. The everyday commands

| Command | What it does | Tip |
|---|---|---|
| `cargo check` | Type-check without producing a binary | Your main feedback loop; much faster than `build` |
| `cargo build` | Debug build into `target/debug/` | |
| `cargo build --release` | Optimised build into `target/release/` | Use for benchmarks and shipping |
| `cargo run -- <args>` | Build and run; args after `--` go to your program | `cargo run -p <crate>` in a workspace |
| `cargo test` | Build and run every test | See section 7 for filters |
| `cargo clippy` | Lint | `cargo clippy --fix` applies safe suggestions |
| `cargo fmt` | Format all code | `cargo fmt --check` in CI |
| `cargo doc --open` | Generate HTML docs | `///` doc comments become docs |
| `cargo add <crate>` / `cargo remove` | Edit dependencies | `cargo add tokio --features full` |
| `cargo tree` | Dependency tree | `cargo tree -i <crate>` shows who pulls it in |
| `cargo update` | Update `Cargo.lock` within SemVer ranges | |
| `cargo clean` | Delete `target/` | `target/` grows large; this is safe |
| `rustc --explain E0382` | Long explanation of an error code | Use it every time |

## 6. Dependencies and crates.io

- Packages come from <https://crates.io>; every published crate has auto-generated docs on <https://docs.rs>.
- Version requirements are SemVer: `"1.2"` means `>=1.2.0, <2.0.0`. `Cargo.lock` pins exact versions.
- **Features** turn on optional parts of a crate: `tokio = { version = "1", features = ["full"] }`. Fewer features means faster compiles and a smaller attack surface.
- Other sources: `{ git = "https://github.com/org/repo", tag = "v1.0" }` or `{ path = "../other-crate" }`. This repo's Lab 8.1 uses a path dependency on Lab 3.3.
- Choosing crates: check downloads, recent releases, and docs on docs.rs. Some de facto standards: `serde` (serialisation), `tokio` (async runtime), `clap` (CLI arguments), `anyhow` / `thiserror` (errors), `tracing` (logging), `reqwest` (HTTP client), `axum` (web), `sqlx` (SQL), `rayon` (data parallelism), `regex`.

## 7. Testing

Three kinds of tests, all run by `cargo test`.

```rust
// 1. Unit tests: next to the code, can test private functions
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]                  // only compiled for `cargo test`
mod tests {
    use super::*;

    #[test]
    fn adds() { assert_eq!(add(2, 2), 4); }

    #[test]
    #[should_panic(expected = "overflow")]
    fn overflows() { let _ = add(i32::MAX, 1); }
}
```

```rust
// 2. Integration tests: tests/*.rs, use only the PUBLIC API (this repo's assessments)
use my_crate::add;

#[test]
fn public_api() { assert_eq!(add(1, 1), 2); }
```

```rust
/// 3. Doc tests: code in doc comments is compiled and run, so docs never go stale.
/// ```
/// assert_eq!(my_crate::add(1, 2), 3);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

**Running a subset**

```bash
cargo test -p lab-2-2-word-tools           # one crate in a workspace
cargo test -p lab-2-2-word-tools longest   # only tests whose name contains "longest"
cargo test -- --nocapture                  # show println! output
cargo test -- --test-threads=1             # run serially
cargo test --release                       # optimised (for heavy tests)
```

Async tests use `#[tokio::test]` (Module 10).

## 8. Build profiles: debug vs release

| | `cargo build` (dev) | `cargo build --release` |
|---|---|---|
| Optimisation | none (`opt-level = 0`) | full (`opt-level = 3`) |
| Debug assertions and integer-overflow checks | on (overflow panics) | off (overflow wraps) |
| Compile speed | fast | slower |
| Runtime speed | often 10–100× slower | production speed |

**Never benchmark a debug build.** Tune the profiles in the root `Cargo.toml`:

```toml
[profile.release]
lto = "fat"          # link-time optimisation across crates: smaller, faster, slower to build
codegen-units = 1    # better optimisation, slower build
strip = true         # remove symbols: much smaller binary
panic = "abort"      # no unwinding: smaller binary (but you can't catch panics)

[profile.dev.package."*"]
opt-level = 2        # optimise dependencies even in debug builds: faster dev runtime
```

## 9. Workspaces (how this repo is organised)

The root `Cargo.toml` here is a **virtual manifest**: it has no package of its own, only a list of members.

```toml
[workspace]
resolver = "3"
members = ["labs/*/lab-*", "capstones/a-rgrep", "capstones/b-mini-redis"]

[workspace.package]
edition = "2024"

[workspace.dependencies]      # versions declared once…
tokio = { version = "1.54", features = ["full"] }
```

Each lab inherits from it:

```toml
[package]
name = "lab-10-2-chat-server"
edition.workspace = true

[dependencies]
tokio = { workspace = true }  # …and used everywhere with the same version
```

Useful workspace commands:

```bash
cargo build                    # everything
cargo test -p lab-3-3-tokenizer
cargo run  -p lab-1-4-base-converter -- 255 hex
cargo test --workspace         # all labs (will show your TODOs as failures)
cargo metadata --format-version 1 | jq '.workspace_members'   # if you have jq
```

All crates share one `target/` directory, so a dependency is compiled once for the whole repo.

## 10. Features and conditional compilation

Features are named compile-time flags for your own crate. Several labs use one called `broken`:

```toml
[features]
broken = []
default = []
```

```rust
#[cfg(feature = "broken")]   // only compiled with --features broken
pub mod broken;

#[cfg(debug_assertions)]     // only in debug builds
fn expensive_invariant_check() {}

#[cfg(target_os = "linux")]  // platform-specific code
fn linux_only() {}
```

```bash
cargo build -p lab-2-1-borrow-checker --features broken
```

## 11. Packaging and distribution

**A. Ship a binary (most common for services and CLIs)**

```bash
cargo build --release
ls -lh target/release/rgrep     # one self-contained executable, no runtime needed
```

Rust binaries link the standard library statically and depend only on the system C library (libc). Copy the file and run it.

**B. Fully static Linux binary (no libc dependency at all)**

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl -p capstone-a-rgrep
# → target/x86_64-unknown-linux-musl/release/rgrep  (runs on any x86-64 Linux, even `FROM scratch`)
```

Cross-compiling to other platforms: `rustup target list`, then `rustup target add <triple>`. For targets that need a foreign C toolchain, the `cross` tool (`cargo install cross`) does the build inside Docker.

**C. Docker multi-stage build**

```dockerfile
FROM rust:1 AS build
WORKDIR /app
COPY . .
RUN cargo build --release -p capstone-b-mini-redis

FROM debian:stable-slim
COPY --from=build /app/target/release/mini-redis /usr/local/bin/mini-redis
EXPOSE 6380
CMD ["mini-redis", "--addr", "0.0.0.0:6380"]
```

With the musl target from B you can use `FROM scratch` and get an image of just a few MB.

**D. Install from source (for you or your users)**

```bash
cargo install --path capstones/a-rgrep          # from a local checkout
cargo install --git https://github.com/<you>/<repo> capstone-a-rgrep
cargo install ripgrep                           # from crates.io
```

**E. Publish a library or tool to crates.io**

```toml
[package]
name = "your-unique-name"
version = "0.1.0"
edition = "2024"
description = "One line about what it does"
license = "MIT OR Apache-2.0"
repository = "https://github.com/you/repo"
readme = "README.md"
keywords = ["cli", "search"]
```

```bash
cargo login                    # API token from crates.io/settings/tokens
cargo package --list           # which files would be uploaded
cargo publish --dry-run        # builds the package exactly as crates.io would
cargo publish                  # irreversible: a version can be yanked but never deleted
```

The labs here set `publish = false` so they can't be published by accident.

**F. Versioning:** follow SemVer. Before 1.0, a breaking change bumps the minor version (`0.3 → 0.4`). Tools such as `cargo-release` and `release-plz` automate version bumps, tags and changelogs.

## 12. Useful extra tools

| Tool | Install | Why |
|---|---|---|
| `bacon` | `cargo install --locked bacon` | Re-runs check/clippy/tests on every save, in a terminal pane |
| `cargo-nextest` | `cargo install --locked cargo-nextest` | Faster test runner with clearer output: `cargo nextest run` |
| `cargo-expand` | `cargo install cargo-expand` | See what macros (`#[derive]`, `println!`) expand to |
| `cargo-audit` | `cargo install cargo-audit` | Check dependencies against the RustSec advisory database |
| `cargo-deny` | `cargo install --locked cargo-deny` | Licence, duplicate and advisory policy for CI |
| `cargo-outdated` | `cargo install cargo-outdated` | Dependencies with newer versions available |
| `hyperfine` | `cargo install hyperfine` | Command-line benchmarking (Capstone A) |
| `cargo-flamegraph` | `cargo install flamegraph` | CPU flame graphs |

## 13. Translation table from your languages

| Task | Rust | Python | TypeScript | Java | Go |
|---|---|---|---|---|---|
| New project | `cargo new app` | `uv init` | `npm init` | `mvn archetype:generate` | `go mod init` |
| Add dependency | `cargo add serde` | `uv add pydantic` | `npm i zod` | edit `pom.xml` | `go get` |
| Manifest | `Cargo.toml` | `pyproject.toml` | `package.json` | `pom.xml` / `build.gradle` | `go.mod` |
| Lock file | `Cargo.lock` | `uv.lock` | `package-lock.json` | — | `go.sum` |
| Run | `cargo run` | `python main.py` | `npx tsx main.ts` | `mvn exec:java` | `go run .` |
| Test | `cargo test` | `pytest` | `vitest` | `mvn test` | `go test ./...` |
| Lint | `cargo clippy` | `ruff check` | `eslint` | SpotBugs / Error Prone | `go vet` |
| Format | `cargo fmt` | `ruff format` / `black` | `prettier` | google-java-format | `gofmt` |
| Docs | `cargo doc --open` | Sphinx | TypeDoc | `javadoc` | `go doc` |
| Release build | `cargo build --release` | — | `tsc` + bundler | `mvn package` | `go build` |
| Registry | crates.io | PyPI | npm | Maven Central | proxy.golang.org |

---

**Next:** go back to the [README](README.md) and start Module 1.
