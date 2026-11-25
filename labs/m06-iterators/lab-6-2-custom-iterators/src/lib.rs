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
        // SOLUTION-START
        let current = self.a;
        if current == 0 {
            return None; // marks "overflowed": no more values
        }
        // checked_add: when the next value would overflow, store 0 as a sentinel
        let next = self.a.checked_add(self.b).unwrap_or(0);
        self.a = self.b;
        self.b = next;
        Some(current)
        // SOLUTION-END
    }
}

/// Sum of the even Fibonacci numbers below `limit`. Adapters only.
pub fn even_fib_sum_below(limit: u64) -> u64 {
    // SOLUTION-START
    fib()
        .take_while(|&n| n < limit)
        .filter(|n| n % 2 == 0)
        .sum()
    // SOLUTION-END
}

/// The first Fibonacci number with exactly `digits` decimal digits.
pub fn first_fib_with_digits(digits: u32) -> Option<u64> {
    // SOLUTION-START
    fib().find(|n| n.to_string().len() as u32 == digits)
    // SOLUTION-END
}

/// Like `(start..end).step_by(step)` but supports negative steps:
/// `step_range(10, 0, -3)` yields 10, 7, 4, 1. A step of 0 yields nothing.
/// Hint: `std::iter::successors` or a small struct.
pub fn step_range(start: i64, end: i64, step: i64) -> impl Iterator<Item = i64> {
    // SOLUTION-START: std::iter::empty() // TODO: replace with your iterator
    std::iter::successors(Some(start), move |&x| x.checked_add(step))
        .take_while(move |&x| step != 0 && if step > 0 { x < end } else { x > end })
    // SOLUTION-END
}

/// Sum of every window of `k` consecutive elements. Use `slice::windows`.
pub fn window_sums(v: &[i32], k: usize) -> Vec<i32> {
    // SOLUTION-START
    if k == 0 {
        return Vec::new();
    }
    v.windows(k).map(|w| w.iter().sum()).collect()
    // SOLUTION-END
}
