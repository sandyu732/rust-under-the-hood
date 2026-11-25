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
    // SOLUTION-START: std::iter::empty() // TODO: replace with your iterator chain
    text.split(|c: char| !(c.is_alphanumeric() || c == '\''))
        .map(|w| w.trim_matches('\''))
        .filter(|w| !w.is_empty())
        .map(str::to_lowercase)
    // SOLUTION-END
}

/// Count every word. Use the entry API: `*map.entry(k).or_insert(0) += 1`.
pub fn word_frequencies(text: &str) -> HashMap<String, usize> {
    // SOLUTION-START
    let mut counts = HashMap::new();
    for w in words(text) {
        *counts.entry(w).or_insert(0) += 1;
    }
    counts
    // SOLUTION-END
}

/// The `n` most frequent words, by count descending, then alphabetically.
pub fn top_n(freqs: &HashMap<String, usize>, n: usize) -> Vec<(&str, usize)> {
    // SOLUTION-START
    let mut all: Vec<(&str, usize)> = freqs.iter().map(|(w, c)| (w.as_str(), *c)).collect();
    all.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    all.truncate(n);
    all
    // SOLUTION-END
}
