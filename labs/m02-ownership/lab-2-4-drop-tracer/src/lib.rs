//! Lab 2.4 — Drop tracer.
//!
//! 1. Implement `Drop` for `Noisy` so it records its name with `log_drop`.
//! 2. Read `scenario()` and PREDICT the order in which values are dropped.
//!    Write it in `predicted_order()`. Don't run it first!
//! 3. `cargo test -p lab-2-4-drop-tracer` checks your prediction.

use std::cell::RefCell;

thread_local! {
    static LOG: RefCell<Vec<&'static str>> = const { RefCell::new(Vec::new()) };
}

/// Record that a value named `name` was dropped.
pub fn log_drop(name: &'static str) {
    LOG.with(|l| l.borrow_mut().push(name));
}

/// Return and clear the drop log.
pub fn take_log() -> Vec<&'static str> {
    LOG.with(|l| std::mem::take(&mut *l.borrow_mut()))
}

pub struct Noisy(pub &'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        // SOLUTION-START: // call log_drop with this value's name
        log_drop(self.0);
        // SOLUTION-END
    }
}

pub fn consume(n: Noisy) {
    let _ = n.0.len();
}

/// Given. Do not change.
#[allow(clippy::useless_vec)] // the Vec is the point: watch how it drops its elements
pub fn scenario() {
    let _a = Noisy("a");
    {
        let _b = Noisy("b");
        let _c = Noisy("c");
    }
    let d = Noisy("d");
    consume(d);
    let _v = vec![Noisy("e"), Noisy("f")];
    let g = Noisy("g");
    drop(g);
    let _h = Noisy("h");
}

/// Your prediction of the drop order of `scenario()`.
pub fn predicted_order() -> Vec<&'static str> {
    // SOLUTION-START
    // inner scope drops in reverse declaration order: c, b
    // d moves into `consume` and drops at the end of it
    // g is dropped explicitly
    // at the end of scenario, locals drop in reverse: _h, then _v (elements in
    // order e, f), then _a
    vec!["c", "b", "d", "g", "h", "e", "f", "a"]
    // SOLUTION-END
}
