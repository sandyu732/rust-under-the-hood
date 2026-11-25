#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 6.5 — Closures: Fn, FnMut, FnOnce, and returning closures.
//!
//! `Fn`     — only reads what it captured; can be called many times, even concurrently
//! `FnMut`  — mutates what it captured; needs `&mut` access to call
//! `FnOnce` — may consume what it captured; can be called once
//! Every Fn is also FnMut and FnOnce, so `FnOnce` is the most permissive bound to ACCEPT.

pub fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    todo!("lab-6-5-closures: implement me")
}

pub fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    todo!("lab-6-5-closures: implement me")
}

/// Call `f` `n` times. `f` may mutate its captured state.
pub fn call_n_times<F: FnMut()>(mut f: F, n: usize) {
    todo!("lab-6-5-closures: implement me")
}

/// Call `f` once and return its result. `f` may move out of its captures.
pub fn run_once<F: FnOnce() -> String>(f: F) -> String {
    todo!("lab-6-5-closures: implement me")
}

/// Each call returns the next number: 1, 2, 3, ...
/// The counter lives inside the closure (`move` it in).
pub fn make_counter() -> impl FnMut() -> u32 {
    || -> u32 { todo!("lab-6-5-closures: implement me") }
}

pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    |_x: i32| -> i32 { todo!("lab-6-5-closures: implement me") }
}

/// `compose(f, g)(x) == g(f(x))`
pub fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |_x: A| -> C {
        let _ = (&f, &g);
        todo!("lab-6-5-closures: implement me")
    }
}
