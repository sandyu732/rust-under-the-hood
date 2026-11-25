#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 2.2 — Zero-copy word tools.
//!
//! Every function takes `&str` and returns either a number or a `&str` that
//! points INTO the input. No allocation allowed: no `String::from`,
//! `.to_string()`, `.to_owned()`, `format!` or `collect::<String>()`.
//! The assessment checks that returned slices live inside the input's memory.

/// The first whitespace-separated word, or "" if there is none.
pub fn first_word(s: &str) -> &str {
    todo!("lab-2-2-word-tools: implement me")
}

/// The longest word. On a tie, the first one wins. "" if there is none.
pub fn longest_word(s: &str) -> &str {
    todo!("lab-2-2-word-tools: implement me")
}

pub fn word_count(s: &str) -> usize {
    todo!("lab-2-2-word-tools: implement me")
}

/// Remove leading/trailing ASCII punctuation: "(hello!)" → "hello".
pub fn trim_punctuation(word: &str) -> &str {
    todo!("lab-2-2-word-tools: implement me")
}
