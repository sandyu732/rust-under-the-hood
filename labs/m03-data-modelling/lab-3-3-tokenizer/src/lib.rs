#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 3.3 — Tokenizer. (Module 8's calculator builds on this crate — keep it!)
//!
//! `"3 + 4.5 * (2 - 1)"` → `[Num(3.0), Plus, Num(4.5), Star, LParen, Num(2.0), Minus, Num(1.0), RParen]`
//!
//! Hint: `let mut chars = input.char_indices().peekable();` then
//! `while let Some((pos, ch)) = chars.next() { match ch { ... } }`.
//! For numbers, keep consuming while `chars.peek()` is a digit or '.'.

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenizeError {
    /// A character that isn't part of any token. `pos` is its byte index.
    UnexpectedChar { ch: char, pos: usize },
    /// Something like "1.2.3". `pos` is the byte index where it starts.
    BadNumber { text: String, pos: usize },
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizeError> {
    todo!("lab-3-3-tokenizer: implement me")
}
