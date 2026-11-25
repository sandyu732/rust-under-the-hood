# Rust Hands-On Labs

A lab-first path to learning Rust, written for experienced developers who already know Python, TypeScript, Java or Go. It has 11 modules, 44 labs and 2 capstone projects with tests, plus 3 open-ended capstone specs. **Every lab from Module 1 on (41 of them) comes with an assessment test suite**: you're done when the tests pass.

```text
Module 1  Primitives ─▶ 2 Ownership ─▶ 3 Enums & matching ─▶ 4 Errors ─▶ 5 Traits
       ─▶ 6 Iterators & closures ─▶ 7 Lifetimes ─▶ 8 Smart pointers
       ─▶ 9 Threads ─▶ 10 Async (Tokio, axum, sqlx) ─▶ Capstones
```

## Quick start

```bash
# 1. Install Rust (see GETTING_STARTED.md for Windows and details)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Clone and build everything once (downloads dependencies, ~2–4 minutes)
git clone https://github.com/sandyu732/rust-under-the-hood.git
cd rust-under-the-hood
cargo build

# 3. See where you stand: every lab starts as TODO
./scripts/progress.sh

# 4. Start the first lab
cargo test -p lab-1-1-type-explorer
```

New to Cargo? Read **[GETTING_STARTED.md](GETTING_STARTED.md)** first. It covers installing the toolchain, project layout, the everyday commands, dependencies, testing, debug vs release builds, workspaces, features, and packaging and distribution (release binaries, static musl builds, Docker, `cargo install`, publishing to crates.io).

## How each lab works

1. Read the module's `README.md` (a short concept recap and the lab list).
2. Open the lab's `src/lib.rs`. Signatures, types and doc comments are given; the bodies are `todo!()`.
3. Implement until the assessment passes:
   ```bash
   cargo test -p lab-2-2-word-tools            # the whole lab
   cargo test -p lab-2-2-word-tools longest    # one test while you iterate
   ```
4. Make it clean: `cargo clippy -p <lab>` and `cargo fmt`. Delete the `#![allow(unused)]` line at the top of the file.
5. Answer the module's `CHECKPOINT.md` questions, and fill in any `NOTES.md` in the lab.
6. Commit. `./scripts/progress.sh` shows your scoreboard.

Some labs are about reading compiler errors, not writing code. They have a `broken` feature:

```bash
cargo build -p lab-2-1-borrow-checker --features broken   # see the errors you're meant to study
```

**Rules that make the labs worth doing**

- Don't edit `tests/`. The tests are the assessment.
- Read every compiler error to the end, and run `rustc --explain <CODE>` on any you don't recognise.
- From Module 4 on, no `.unwrap()` in library code unless you can say why it can't fail.
- Stuck on the borrow checker for more than 20 minutes? `.clone()` your way through, make the tests pass, then come back and remove the clone.

## Reference solutions

The `solutions` branch has a working implementation of every lab, answers to every checkpoint and notes file, and the tool that generates `main` from it.

```bash
git fetch origin solutions
git diff main origin/solutions -- labs/m02-ownership/lab-2-2-word-tools   # compare one lab
git show origin/solutions:labs/m02-ownership/CHECKPOINT.md               # read answers
```

Finish the lab before you look. Your solution doesn't have to match; it has to pass and be clippy-clean.

## Modules

| # | Module | Labs | Key ideas |
|---|---|---|---|
| 0 | [Setup](labs/m00-setup) | 3 | rustup, cargo, workspace, reading errors |
| 1 | [Primitives](labs/m01-primitives) | 4 | sized integers, overflow, `as`/`TryFrom`, expressions |
| 2 | [Ownership](labs/m02-ownership) | 4 | move, borrow, `&`/`&mut`, slices, `Drop` |
| 3 | [Data modelling](labs/m03-data-modelling) | 4 | structs, enums with data, `match`, newtypes, modules |
| 4 | [Errors](labs/m04-errors) | 4 | `Result`, `?`, custom errors, `thiserror`, `anyhow` |
| 5 | [Traits & generics](labs/m05-traits) | 5 | `dyn` vs generics, operator traits, blanket impls |
| 6 | [Iterators & closures](labs/m06-iterators) | 5 | adapters, custom `Iterator`, `Fn`/`FnMut`/`FnOnce` |
| 7 | [Lifetimes](labs/m07-lifetimes) | 3 | annotations, borrowed structs, `Cow` |
| 8 | [Smart pointers](labs/m08-smart-pointers) | 4 | `Box`, `Rc`/`Weak`, `RefCell`, arenas |
| 9 | [Concurrency](labs/m09-concurrency) | 4 | `Send`/`Sync`, `Arc<Mutex>`, scoped threads, channels, rayon |
| 10 | [Async](labs/m10-async) | 4 | Tokio, `select!`, bounded concurrency, axum, sqlx |
| ★ | [Capstones](capstones) | 2 + 3 | rgrep, mini-Redis, production API, Wasm, PyO3 |

**Suggested pace:** about 6–8 hours a week. Week 1: modules 0–1 · Week 2: module 2 · Week 3: modules 3–4 · Week 4: modules 5–6 · Week 5: modules 7–8 · Week 6: module 9 · Week 7: module 10 · Week 8 and on: a capstone.

**Parallel track:** [Rustlings](https://github.com/rust-lang/rustlings) (`cargo install rustlings && rustlings init`). Each module README lists the matching Rustlings sections.

## Repository layout

```text
.
├── Cargo.toml              # workspace: members + shared dependency versions
├── rust-toolchain.toml     # pins stable + clippy + rustfmt for everyone
├── GETTING_STARTED.md      # toolchain, cargo, building, packaging
├── scripts/progress.sh     # runs every assessment and prints a scoreboard
├── labs/
│   └── mNN-topic/
│       ├── README.md       # concept recap + lab list
│       ├── CHECKPOINT.md   # questions to answer before moving on
│       └── lab-N-M-name/
│           ├── Cargo.toml
│           ├── src/lib.rs  # ← you work here
│           ├── src/main.rs # (some labs) a runnable demo/CLI
│           ├── NOTES.md    # (some labs) fill-in notes
│           └── tests/      # the assessment — don't edit
└── capstones/
```

## Further reading

- [The Rust Programming Language](https://doc.rust-lang.org/book/) ("the Book"): read the chapter that matches each module
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/) (Module 8)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial) (Module 10 and Capstone B)
- *Zero To Production in Rust* by Luca Palmieri (Capstone C)
- *Rust for Rustaceans* by Jon Gjengset (after this path)
- *Rust Atomics and Locks* by Mara Bos (if Module 9 hooks you)
