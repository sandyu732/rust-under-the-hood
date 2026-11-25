# Module 7 — Lifetimes

**When:** Week 5

Lifetimes are how the compiler proves a reference never outlives its data. You'll rarely write them, because most are inferred, but you need to be able to read them and understand the errors.

## Core concepts

- `'a` doesn't change how long anything lives. It *describes a relationship* between references.
- Elision: one input reference gives the output its lifetime; a `&self` method gives the output `self`'s lifetime.
- Annotate when the output could come from one of several inputs, and whenever a struct holds a reference.
- `T: 'static` means "contains no short-lived borrows", not "lives forever".
- Pragmatic rule: if borrowed structs get painful, own the data. Borrowing is an optimisation, best suited to parsers and views.

## Labs

| Lab | Crate | Done when |
|---|---|---|
| **7.1** Meet the error | `lab-7-1-longest` | `NOTES.md` explains E0106 and E0597 (built with `--features broken`) and the tests pass. |
| **7.2** Zero-copy HTTP parser | `lab-7-2-http-parser` | The tests pass, including the zero-copy pointer checks. |
| **7.3** Owned vs borrowed | `lab-7-3-owned-vs-borrowed` | Both fixes for E0515 work (built with `--features broken`) and the `Cow` tests pass. |

```bash
cargo test -p lab-7-1-longest
cargo test -p lab-7-2-http-parser
cargo test -p lab-7-3-owned-vs-borrowed
./scripts/progress.sh m07   # this module's scoreboard
```

## Rustlings (parallel track)

Do these sections after this module: `16_lifetimes`.

## Checkpoint

Answer the questions in [CHECKPOINT.md](CHECKPOINT.md) before moving on.
