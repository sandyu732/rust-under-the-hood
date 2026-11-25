#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 9.1 — Let the compiler stop your data races.
//!
//! 1. `cargo build -p lab-9-1-shared-counter --features broken` and read the
//!    errors in `src/broken.rs`. Note which trait (`Send` / `Sync`) is missing
//!    or which lifetime problem appears. Fill in NOTES.md.
//! 2. Implement the three correct versions below.

#[cfg(feature = "broken")]
pub mod broken;

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

/// `threads` threads each increment a shared counter `per_thread` times.
/// Use `Arc<Mutex<u64>>` + `thread::spawn` + `join`.
pub fn count_with_arc_mutex(threads: usize, per_thread: u64) -> u64 {
    todo!("lab-9-1-shared-counter: implement me")
}

/// Same, but with `thread::scope` — threads may BORROW a local `Mutex`, so no `Arc`.
pub fn count_with_scope(threads: usize, per_thread: u64) -> u64 {
    todo!("lab-9-1-shared-counter: implement me")
}

/// Same, lock-free with an `AtomicU64` (`fetch_add` with `Ordering::Relaxed`
/// is enough for a pure counter).
pub fn count_with_atomic(threads: usize, per_thread: u64) -> u64 {
    todo!("lab-9-1-shared-counter: implement me")
}

/// Split `data` into `chunks` roughly equal slices, sum each on its own
/// scoped thread, and add the partial sums. (`data.chunks(size)`.)
pub fn parallel_sum(data: &[u64], chunks: usize) -> u64 {
    todo!("lab-9-1-shared-counter: implement me")
}
