#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 6.4 — Zero-cost abstractions: prove iterators are as fast as loops.
//!
//! Implement the same computation three ways, then:
//!   cargo run --release -p lab-6-4-zero-cost
//! Optional: paste the functions into https://godbolt.org (rustc, -O) and
//! compare the assembly.

/// Sum of squares of the odd numbers, using `while i < v.len()` and `v[i]`.
pub fn sum_odd_squares_index(v: &[u64]) -> u64 {
    todo!("lab-6-4-zero-cost: implement me")
}

/// Same, with `for &x in v`.
pub fn sum_odd_squares_for(v: &[u64]) -> u64 {
    todo!("lab-6-4-zero-cost: implement me")
}

/// Same, as one iterator chain: `filter` → `map` → `sum`.
pub fn sum_odd_squares_iter(v: &[u64]) -> u64 {
    todo!("lab-6-4-zero-cost: implement me")
}
