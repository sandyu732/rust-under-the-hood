#![allow(unused)] // TODO: delete this line once the lab is done
//! Fixed versions of `broken.rs`. Same behaviour, but they compile.

/// Returns the combined length of the two strings ("hi" twice → 4).
pub fn a_use_after_move() -> usize {
    todo!("lab-2-1-borrow-checker: implement me")
}

/// Returns `[1, 2, 3, 4, 5]`.
pub fn b_two_mutable_borrows() -> Vec<i32> {
    todo!("lab-2-1-borrow-checker: implement me")
}

/// Returns `(10, 4)`: the original first element and the new length.
pub fn c_push_while_borrowed() -> (i32, usize) {
    todo!("lab-2-1-borrow-checker: implement me")
}

/// Returns "hello". Note the return type had to change: you can't return a
/// reference to something that dies at the end of the function.
pub fn d_dangling() -> String {
    todo!("lab-2-1-borrow-checker: implement me")
}

/// For every even element, append `element * 10`. `[1, 2, 3, 4]` → `[1, 2, 3, 4, 20, 40]`.
pub fn e_mutate_while_iterating(v: &mut Vec<i32>) {
    todo!("lab-2-1-borrow-checker: implement me")
}
