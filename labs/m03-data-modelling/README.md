# Module 3 — Structs, enums, pattern matching

**When:** Week 3

Rust enums are algebraic data types: like TypeScript discriminated unions, but checked exhaustively by the compiler. Together with `match`, they let you make illegal states unrepresentable instead of guarding against them at runtime.

## Core concepts

- Structs: named, tuple (`struct Meters(f64);`) and unit. Constructors are just associated functions, by convention `fn new(...) -> Self`.
- Method receivers show ownership: `&self` reads, `&mut self` modifies, `self` consumes.
- `#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]` gets you common behaviour for free.
- `match` must be exhaustive. Patterns: destructuring, guards, ranges, `@` bindings, `if let`, `let … else`, `matches!`.
- Everything is private by default. Modules (`mod`) plus `pub` are your encapsulation.

## Labs

| Lab | Crate | Done when |
|---|---|---|
| **3.1** Shapes | `lab-3-1-shapes` | The tests pass. Then add a `Square` variant and let the compiler list every `match` you need to update. |
| **3.2** Order state machine | `lab-3-2-order-state` | The tests pass, and it's impossible to ship a rejected order. |
| **3.3** Tokenizer | `lab-3-3-tokenizer` | The tests pass, including error positions. **Lab 8.1 depends on this crate.** |
| **3.4** Newtype money | `lab-3-4-money` | The tests pass, and code outside `bank` can't touch `cents` (try it and read the error). |

```bash
cargo test -p lab-3-1-shapes
cargo test -p lab-3-2-order-state
cargo test -p lab-3-3-tokenizer
cargo test -p lab-3-4-money
./scripts/progress.sh m03   # this module's scoreboard
```

## Rustlings (parallel track)

Do these sections after this module: `07_structs`, `08_enums`, `10_modules`.

## Checkpoint

Answer the questions in [CHECKPOINT.md](CHECKPOINT.md) before moving on.
