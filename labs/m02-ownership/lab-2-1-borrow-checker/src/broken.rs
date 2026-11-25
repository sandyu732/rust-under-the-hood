//! Each function below fails to compile. Do not fix them here — fix them in
//! `fixed.rs`. This file is only compiled with `--features broken`.

/// (a) Use after move.
pub fn a_use_after_move() -> usize {
    let s = String::from("hi");
    let t = s;
    s.len() + t.len()
}

/// (b) Two mutable borrows alive at once.
pub fn b_two_mutable_borrows() -> Vec<i32> {
    let mut v = vec![1, 2, 3];
    let a = &mut v;
    let b = &mut v;
    a.push(4);
    b.push(5);
    v
}

/// (c) Pushing to a Vec while holding a reference into it.
///     (In Java this compiles and may throw at runtime, or silently read stale data.)
pub fn c_push_while_borrowed() -> (i32, usize) {
    let mut v = vec![10, 20, 30];
    let first = &v[0];
    v.push(40);
    (*first, v.len())
}

/// (d) Returning a reference to a local variable.
pub fn d_dangling() -> &String {
    let s = String::from("hello");
    &s
}

/// (e) Modifying a Vec while iterating over it.
pub fn e_mutate_while_iterating(v: &mut Vec<i32>) {
    for x in v.iter() {
        if *x % 2 == 0 {
            v.push(*x * 10);
        }
    }
}
