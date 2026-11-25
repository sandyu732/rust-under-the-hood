#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 6.2 — Implement `Iterator` and get ~70 adapter methods for free.

/// Yields 1, 1, 2, 3, 5, 8, ... (stops before overflowing u64).
pub struct Fib {
    a: u64,
    b: u64,
}

pub fn fib() -> Fib {
    Fib { a: 1, b: 1 }
}

impl Iterator for Fib {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        todo!("lab-6-2-custom-iterators: implement me")
    }
}

/// Sum of the even Fibonacci numbers below `limit`. Adapters only.
pub fn even_fib_sum_below(limit: u64) -> u64 {
    todo!("lab-6-2-custom-iterators: implement me")
}

/// The first Fibonacci number with exactly `digits` decimal digits.
pub fn first_fib_with_digits(digits: u32) -> Option<u64> {
    todo!("lab-6-2-custom-iterators: implement me")
}

/// Like `(start..end).step_by(step)` but supports negative steps:
/// `step_range(10, 0, -3)` yields 10, 7, 4, 1. A step of 0 yields nothing.
/// Hint: `std::iter::successors` or a small struct.
pub fn step_range(start: i64, end: i64, step: i64) -> impl Iterator<Item = i64> {
    std::iter::empty() // TODO: replace with your iterator
}

/// Sum of every window of `k` consecutive elements. Use `slice::windows`.
pub fn window_sums(v: &[i32], k: usize) -> Vec<i32> {
    todo!("lab-6-2-custom-iterators: implement me")
}
