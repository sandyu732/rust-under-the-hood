# Module 8 — Smart pointers and interior mutability

**When:** Week 5

Ownership's one-owner rule doesn't fit every data structure: trees with parent links, graphs, shared caches. Smart pointers are the controlled escape hatches, and each has an explicit, visible cost.

## Core concepts

- `Box<T>`: single owner on the heap. Needed for recursive types and trait objects.
- `Rc<T>` / `Arc<T>`: shared ownership via reference counts (non-atomic / atomic). `Weak<T>` breaks cycles.
- `Cell<T>` / `RefCell<T>`: mutation through `&`, with the borrow rules checked at **runtime** (RefCell panics on a violation).
- Shared + mutable: `Rc<RefCell<T>>` on a single thread, `Arc<Mutex<T>>` across threads.
- Often the better design is an **arena**: a `Vec` that owns every node, with plain index links between them.

## Labs

| Lab | Crate | Done when |
|---|---|---|
| **8.1** Calculator | `lab-8-1-calculator` | `2 * (3 + 4) - -1` gives `15`, and division by zero is an error. Requires Lab 3.3. `cargo run -p lab-8-1-calculator` starts the REPL. |
| **8.2** Linked list | `lab-8-2-linked-list` | Both test files pass, including the million-node drop test (write the iterative `Drop` yourself). |
| **8.3** Rc tree with Weak parents | `lab-8-3-rc-tree` | The tests pass, and you can explain why `parent` must be `Weak`. |
| **8.4** Arena tree | `lab-8-4-arena-tree` | The same answers as 8.3 with no `Rc` or `RefCell`. Note which version you'd rather maintain. |

```bash
cargo test -p lab-8-1-calculator
cargo test -p lab-8-2-linked-list
cargo test -p lab-8-3-rc-tree
cargo test -p lab-8-4-arena-tree
./scripts/progress.sh m08   # this module's scoreboard
```

## Rustlings (parallel track)

Do these sections after this module: `19_smart_pointers`.

## Checkpoint

Answer the questions in [CHECKPOINT.md](CHECKPOINT.md) before moving on.
