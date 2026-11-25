//! Lab 6.5 — Closures: Fn, FnMut, FnOnce, and returning closures.
//!
//! `Fn`     — only reads what it captured; can be called many times, even concurrently
//! `FnMut`  — mutates what it captured; needs `&mut` access to call
//! `FnOnce` — may consume what it captured; can be called once
//! Every Fn is also FnMut and FnOnce, so `FnOnce` is the most permissive bound to ACCEPT.

pub fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    // SOLUTION-START
    f(x)
    // SOLUTION-END
}

pub fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    // SOLUTION-START
    f(f(x))
    // SOLUTION-END
}

/// Call `f` `n` times. `f` may mutate its captured state.
pub fn call_n_times<F: FnMut()>(mut f: F, n: usize) {
    // SOLUTION-START
    for _ in 0..n {
        f();
    }
    // SOLUTION-END
}

/// Call `f` once and return its result. `f` may move out of its captures.
pub fn run_once<F: FnOnce() -> String>(f: F) -> String {
    // SOLUTION-START
    f()
    // SOLUTION-END
}

/// Each call returns the next number: 1, 2, 3, ...
/// The counter lives inside the closure (`move` it in).
pub fn make_counter() -> impl FnMut() -> u32 {
    // SOLUTION-START: || -> u32 { todo!("lab-6-5-closures: implement me") }
    let mut count = 0;
    move || {
        count += 1;
        count
    }
    // SOLUTION-END
}

pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    // SOLUTION-START: |_x: i32| -> i32 { todo!("lab-6-5-closures: implement me") }
    move |x| x + n
    // SOLUTION-END
}

/// `compose(f, g)(x) == g(f(x))`
pub fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    // SOLUTION-START: move |_x: A| -> C { let _ = (&f, &g); todo!("lab-6-5-closures: implement me") }
    move |x| g(f(x))
    // SOLUTION-END
}
