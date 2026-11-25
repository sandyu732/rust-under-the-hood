//! Lab 6.4 — Zero-cost abstractions: prove iterators are as fast as loops.
//!
//! Implement the same computation three ways, then:
//!   cargo run --release -p lab-6-4-zero-cost
//! Optional: paste the functions into https://godbolt.org (rustc, -O) and
//! compare the assembly.

/// Sum of squares of the odd numbers, using `while i < v.len()` and `v[i]`.
pub fn sum_odd_squares_index(v: &[u64]) -> u64 {
    // SOLUTION-START
    let mut total = 0;
    let mut i = 0;
    while i < v.len() {
        if v[i] % 2 == 1 {
            total += v[i] * v[i];
        }
        i += 1;
    }
    total
    // SOLUTION-END
}

/// Same, with `for &x in v`.
pub fn sum_odd_squares_for(v: &[u64]) -> u64 {
    // SOLUTION-START
    let mut total = 0;
    for &x in v {
        if x % 2 == 1 {
            total += x * x;
        }
    }
    total
    // SOLUTION-END
}

/// Same, as one iterator chain: `filter` → `map` → `sum`.
pub fn sum_odd_squares_iter(v: &[u64]) -> u64 {
    // SOLUTION-START
    v.iter().filter(|&&x| x % 2 == 1).map(|&x| x * x).sum()
    // SOLUTION-END
}
