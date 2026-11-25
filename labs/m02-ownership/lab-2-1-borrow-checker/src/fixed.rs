//! Fixed versions of `broken.rs`. Same behaviour, but they compile.

/// Returns the combined length of the two strings ("hi" twice → 4).
pub fn a_use_after_move() -> usize {
    // SOLUTION-START
    let s = String::from("hi");
    let t = &s; // borrow instead of move (or `s.clone()` if you need two owners)
    s.len() + t.len()
    // SOLUTION-END
}

/// Returns `[1, 2, 3, 4, 5]`.
pub fn b_two_mutable_borrows() -> Vec<i32> {
    // SOLUTION-START
    let mut v = vec![1, 2, 3];
    {
        let a = &mut v;
        a.push(4);
    } // `a` ends here, so a new &mut is allowed
    let b = &mut v;
    b.push(5);
    v
    // SOLUTION-END
}

/// Returns `(10, 4)`: the original first element and the new length.
pub fn c_push_while_borrowed() -> (i32, usize) {
    // SOLUTION-START
    let mut v = vec![10, 20, 30];
    let first = v[0]; // i32 is Copy: copy the value out instead of borrowing
    v.push(40);
    (first, v.len())
    // SOLUTION-END
}

/// Returns "hello". Note the return type had to change: you can't return a
/// reference to something that dies at the end of the function.
pub fn d_dangling() -> String {
    // SOLUTION-START
    String::from("hello") // return the owned value; ownership moves to the caller
    // SOLUTION-END
}

/// For every even element, append `element * 10`. `[1, 2, 3, 4]` → `[1, 2, 3, 4, 20, 40]`.
pub fn e_mutate_while_iterating(v: &mut Vec<i32>) {
    // SOLUTION-START
    // Collect first (immutable borrow ends), then mutate.
    let extra: Vec<i32> = v.iter().filter(|x| *x % 2 == 0).map(|x| x * 10).collect();
    v.extend(extra);
    // SOLUTION-END
}
