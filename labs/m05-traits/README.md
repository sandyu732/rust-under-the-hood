# Module 5 — Traits and generics

**When:** Week 4

Traits are Rust's only abstraction mechanism: interfaces, operator overloading, conversions, iteration, printing and thread-safety are all traits. Generics are monomorphized, so abstraction costs nothing at runtime (unlike Java/TypeScript type erasure).

## Core concepts

- `trait` + `impl Trait for Type`; traits can have default methods.
- You can implement your own traits for foreign types (`impl Shout for str`). Orphan rule: either the trait or the type must be local.
- Static dispatch (`<T: Trait>`, `impl Trait`): monomorphized and inlinable. Dynamic dispatch (`Box<dyn Trait>`): a vtable, and it allows mixed collections.
- Operators are traits in `std::ops`; conversions are `From`/`Into`/`TryFrom`; ordering is `PartialOrd`/`Ord`.
- Conditional impls (`impl<T: Display> Display for Stack<T>`) and blanket impls (`impl<T: Debug> Describe for T`).

## Labs

| Lab | Crate | Done when |
|---|---|---|
| **5.1** Notifier plugins | `lab-5-1-notifiers` | The tests pass, and you can say when you'd choose `dyn` and when generics. |
| **5.2** Vector math | `lab-5-2-vec2` | `v + v * 3.0 - Vec2::default()` compiles and the tests pass. |
| **5.3** Generic stack | `lab-5-3-stack` | The tests pass with `i32`, `String` and a custom struct. |
| **5.4** Ordering and top-K | `lab-5-4-top-k` | The tests pass, using a heap of size k (O(n log k)). |
| **5.5** Extension traits | `lab-5-5-extension-trait` | `"hi".shout() == "HI!"` and the blanket impl works for any `Debug` type. |

```bash
cargo test -p lab-5-1-notifiers
cargo test -p lab-5-2-vec2
cargo test -p lab-5-3-stack
cargo test -p lab-5-4-top-k
cargo test -p lab-5-5-extension-trait
./scripts/progress.sh m05   # this module's scoreboard
```

## Rustlings (parallel track)

Do these sections after this module: `14_generics`, `15_traits`.

## Checkpoint

Answer the questions in [CHECKPOINT.md](CHECKPOINT.md) before moving on.
