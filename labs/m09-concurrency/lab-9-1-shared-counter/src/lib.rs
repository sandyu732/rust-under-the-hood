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
    // SOLUTION-START
    let counter = Arc::new(Mutex::new(0u64));
    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let c = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..per_thread {
                    *c.lock().unwrap() += 1;
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    *counter.lock().unwrap()
    // SOLUTION-END
}

/// Same, but with `thread::scope` — threads may BORROW a local `Mutex`, so no `Arc`.
pub fn count_with_scope(threads: usize, per_thread: u64) -> u64 {
    // SOLUTION-START
    let counter = Mutex::new(0u64);
    thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                for _ in 0..per_thread {
                    *counter.lock().unwrap() += 1;
                }
            });
        }
    });
    counter.into_inner().unwrap()
    // SOLUTION-END
}

/// Same, lock-free with an `AtomicU64` (`fetch_add` with `Ordering::Relaxed`
/// is enough for a pure counter).
pub fn count_with_atomic(threads: usize, per_thread: u64) -> u64 {
    // SOLUTION-START
    let counter = AtomicU64::new(0);
    thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                for _ in 0..per_thread {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });
    counter.load(Ordering::Relaxed)
    // SOLUTION-END
}

/// Split `data` into `chunks` roughly equal slices, sum each on its own
/// scoped thread, and add the partial sums. (`data.chunks(size)`.)
pub fn parallel_sum(data: &[u64], chunks: usize) -> u64 {
    // SOLUTION-START
    if data.is_empty() {
        return 0;
    }
    let size = data.len().div_ceil(chunks.max(1));
    thread::scope(|s| {
        let handles: Vec<_> = data
            .chunks(size)
            .map(|c| s.spawn(move || c.iter().sum::<u64>()))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    })
    // SOLUTION-END
}
