/// E0106: the compiler can't tell whether the output borrows from `a` or `b`.
pub fn longest(a: &str, b: &str) -> &str {
    if a.len() >= b.len() { a } else { b }
}

/// E0597: `b` is dropped at the end of the inner block while `result`
/// (which might borrow from `b`) is still used afterwards.
pub fn caller() -> usize {
    let a = String::from("a fairly long string");
    let result;
    {
        let b = String::from("short");
        result = crate::longest(&a, &b);
    }
    result.len()
}
