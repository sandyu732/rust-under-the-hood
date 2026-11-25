//! Lab 1.3 — Expression-oriented Rust.
//!
//! Rules for this lab: no `return` keyword, and no `let mut` except where a
//! comment says it's allowed. `if`, `match`, `loop` and blocks are expressions.

/// "Fizz" for multiples of 3, "Buzz" for 5, "FizzBuzz" for both, else the number.
/// Hint: `match (n % 3, n % 5) { (0, 0) => ..., }`
pub fn fizzbuzz(n: u32) -> String {
    // SOLUTION-START
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
    // SOLUTION-END
}

/// FizzBuzz for 1..=n. Use an iterator: `(1..=n).map(...).collect()`.
pub fn fizzbuzz_upto(n: u32) -> Vec<String> {
    // SOLUTION-START
    (1..=n).map(fizzbuzz).collect()
    // SOLUTION-END
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    // SOLUTION-START
    c * 9.0 / 5.0 + 32.0
    // SOLUTION-END
}

pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    // SOLUTION-START
    (f - 32.0) * 5.0 / 9.0
    // SOLUTION-END
}

/// "negative", "zero" or "positive" — a single `if` expression.
pub fn classify(n: i32) -> &'static str {
    // SOLUTION-START
    if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else {
        "positive"
    }
    // SOLUTION-END
}

/// 90..=100 → 'A', 80..=89 → 'B', 70..=79 → 'C', 60..=69 → 'D', else 'F'.
/// Scores above 100 are invalid: return `None`. Use range patterns.
pub fn grade(score: u8) -> Option<char> {
    // SOLUTION-START
    match score {
        90..=100 => Some('A'),
        80..=89 => Some('B'),
        70..=79 => Some('C'),
        60..=69 => Some('D'),
        0..=59 => Some('F'),
        _ => None,
    }
    // SOLUTION-END
}

/// Number of Collatz steps to reach 1 (n even → n/2, odd → 3n+1).
/// Use `loop` + `break <value>`. `let mut` IS allowed here.
pub fn collatz_steps(n: u64) -> u32 {
    // SOLUTION-START
    let mut n = n;
    let mut steps = 0;
    loop {
        if n <= 1 {
            break steps;
        }
        n = if n.is_multiple_of(2) {
            n / 2
        } else {
            3 * n + 1
        };
        steps += 1;
    }
    // SOLUTION-END
}
