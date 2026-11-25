#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 6.1 — Word frequency with HashMap + iterators.
//!
//! `cargo run --release -p lab-6-1-word-freq -- book.txt 20`
//! (Grab any public-domain book from gutenberg.org as a .txt.)
//! Rule: no index-based loops (`for i in 0..n`) — iterators only.

use std::collections::HashMap;

/// Split into lowercase words. A word is a run of alphanumeric chars or
/// apostrophes; everything else separates words. Apostrophes at the edges
/// are trimmed ("'tis" → "tis", "don't" stays).
pub fn words(text: &str) -> impl Iterator<Item = String> + '_ {
    std::iter::empty() // TODO: replace with your iterator chain
}

/// Count every word. Use the entry API: `*map.entry(k).or_insert(0) += 1`.
pub fn word_frequencies(text: &str) -> HashMap<String, usize> {
    todo!("lab-6-1-word-freq: implement me")
}

/// The `n` most frequent words, by count descending, then alphabetically.
pub fn top_n(freqs: &HashMap<String, usize>, n: usize) -> Vec<(&str, usize)> {
    todo!("lab-6-1-word-freq: implement me")
}
