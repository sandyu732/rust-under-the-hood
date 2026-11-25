//! Lab 2.2 — Zero-copy word tools.
//!
//! Every function takes `&str` and returns either a number or a `&str` that
//! points INTO the input. No allocation allowed: no `String::from`,
//! `.to_string()`, `.to_owned()`, `format!` or `collect::<String>()`.
//! The assessment checks that returned slices live inside the input's memory.

/// The first whitespace-separated word, or "" if there is none.
pub fn first_word(s: &str) -> &str {
    // SOLUTION-START
    s.split_whitespace().next().unwrap_or("")
    // SOLUTION-END
}

/// The longest word. On a tie, the first one wins. "" if there is none.
pub fn longest_word(s: &str) -> &str {
    // SOLUTION-START
    s.split_whitespace()
        .fold("", |best, w| if w.len() > best.len() { w } else { best })
    // SOLUTION-END
}

pub fn word_count(s: &str) -> usize {
    // SOLUTION-START
    s.split_whitespace().count()
    // SOLUTION-END
}

/// Remove leading/trailing ASCII punctuation: "(hello!)" → "hello".
pub fn trim_punctuation(word: &str) -> &str {
    // SOLUTION-START
    word.trim_matches(|c: char| c.is_ascii_punctuation())
    // SOLUTION-END
}
