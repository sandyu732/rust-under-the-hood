//! Lab 7.1 — Meet the lifetime errors.
//!
//! 1. `cargo build -p lab-7-1-longest --features broken` — read E0106 and E0597.
//!    Fill in NOTES.md.
//! 2. Implement the functions below. The signatures are given; the point is
//!    to understand WHY each lifetime is where it is. As an experiment, remove
//!    the `'t` from `Highlighter::find_all`'s return type (letting elision tie it
//!    to `&self`) and watch the `results_outlive_the_highlighter` test fail to compile.

#[cfg(feature = "broken")]
pub mod broken;

/// The output borrows from BOTH inputs, so both share `'a`.
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    // SOLUTION-START
    if a.len() >= b.len() { a } else { b }
    // SOLUTION-END
}

/// The output only ever borrows from `a`, so `b` needs no named lifetime —
/// callers may drop `b` while still using the result.
pub fn prefix_before<'a>(a: &'a str, delimiter: &str) -> &'a str {
    // SOLUTION-START
    a.split_once(delimiter).map_or(a, |(head, _)| head)
    // SOLUTION-END
}

/// A struct that borrows text it doesn't own.
pub struct Highlighter<'t> {
    text: &'t str,
}

impl<'t> Highlighter<'t> {
    pub fn new(text: &'t str) -> Self {
        Highlighter { text }
    }

    /// Every occurrence of `needle`, as slices of the ORIGINAL text (`'t`),
    /// not of `self` or `needle`.
    pub fn find_all(&self, needle: &str) -> Vec<&'t str> {
        // SOLUTION-START
        if needle.is_empty() {
            return Vec::new();
        }
        self.text
            .match_indices(needle)
            .map(|(i, m)| &self.text[i..i + m.len()])
            .collect()
        // SOLUTION-END
    }

    /// Elision: `&self` in → output gets self's lifetime. No annotation needed.
    pub fn first_line(&self) -> &str {
        // SOLUTION-START
        self.text.lines().next().unwrap_or("")
        // SOLUTION-END
    }
}
