# Lab 9.1 notes

| Broken fn | Error code(s) | Missing guarantee | Fix |
|---|---|---|---|
| (a) plain counter | <!-- ANSWER-START -->E0597 (`count` does not live long enough: `thread::spawn` requires a `'static` closure, but it borrows the local) and E0503 (using `count` while it is mutably borrowed by the threads)<!-- ANSWER-END --> | <!-- ANSWER-START -->`thread::spawn` needs `'static` closures; and unsynchronised `&mut` from several threads is a data race.<!-- ANSWER-END --> | <!-- ANSWER-START -->`Arc<Mutex<_>>`, or `thread::scope` + `Mutex`, or an `AtomicU64`.<!-- ANSWER-END --> |
| (b) `Rc` across threads | <!-- ANSWER-START -->E0277: `Rc<Vec<i32>>` cannot be sent between threads safely<!-- ANSWER-END --> | <!-- ANSWER-START -->`Send` — Rc's reference count is not atomic, so two threads cloning/dropping could corrupt it.<!-- ANSWER-END --> | <!-- ANSWER-START -->Use `Arc`.<!-- ANSWER-END --> |
| (c) `RefCell` across threads | <!-- ANSWER-START -->E0277: `RefCell<i32>` cannot be shared between threads safely<!-- ANSWER-END --> | <!-- ANSWER-START -->`Sync` — RefCell's borrow flag isn't thread-safe, so `&RefCell` can't be shared; and `Arc<T>` is only `Send` if `T: Send + Sync`.<!-- ANSWER-END --> | <!-- ANSWER-START -->`Mutex` / `RwLock` (thread-safe interior mutability) or an atomic.<!-- ANSWER-END --> |

**Go comparison:** what does `go run -race` report for the Go equivalent of (a)?
<!-- ANSWER-START -->It compiles and runs, producing a wrong (racy) count; only the race detector flags it at runtime, and only if the race actually happens during that run.<!-- ANSWER-END -->
