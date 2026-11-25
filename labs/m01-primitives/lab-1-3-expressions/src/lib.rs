#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 1.3 — Expression-oriented Rust.
//!
//! Rules for this lab: no `return` keyword, and no `let mut` except where a
//! comment says it's allowed. `if`, `match`, `loop` and blocks are expressions.

/// "Fizz" for multiples of 3, "Buzz" for 5, "FizzBuzz" for both, else the number.
/// Hint: `match (n % 3, n % 5) { (0, 0) => ..., }`
pub fn fizzbuzz(n: u32) -> String {
    todo!("lab-1-3-expressions: implement me")
}

/// FizzBuzz for 1..=n. Use an iterator: `(1..=n).map(...).collect()`.
pub fn fizzbuzz_upto(n: u32) -> Vec<String> {
    todo!("lab-1-3-expressions: implement me")
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    todo!("lab-1-3-expressions: implement me")
}

pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    todo!("lab-1-3-expressions: implement me")
}

/// "negative", "zero" or "positive" — a single `if` expression.
pub fn classify(n: i32) -> &'static str {
    todo!("lab-1-3-expressions: implement me")
}

/// 90..=100 → 'A', 80..=89 → 'B', 70..=79 → 'C', 60..=69 → 'D', else 'F'.
/// Scores above 100 are invalid: return `None`. Use range patterns.
pub fn grade(score: u8) -> Option<char> {
    todo!("lab-1-3-expressions: implement me")
}

/// Number of Collatz steps to reach 1 (n even → n/2, odd → 3n+1).
/// Use `loop` + `break <value>`. `let mut` IS allowed here.
pub fn collatz_steps(n: u64) -> u32 {
    todo!("lab-1-3-expressions: implement me")
}
