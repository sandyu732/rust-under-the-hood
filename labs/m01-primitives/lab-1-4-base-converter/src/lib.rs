#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 1.4 — Base converter CLI.
//!
//! `cargo run -p lab-1-4-base-converter -- 255 hex` prints `0xff`.
//! Handle every error with `match` — no `unwrap()`, no `?` (that's Module 4).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base {
    Bin,
    Oct,
    Hex,
}

/// "bin" | "oct" | "hex" (case-insensitive) → `Some(Base)`, anything else → `None`.
pub fn parse_base(s: &str) -> Option<Base> {
    todo!("lab-1-4-base-converter: implement me")
}

/// 255, Hex → "0xff"; 5, Bin → "0b101"; 8, Oct → "0o10".
/// Hint: `format!("{n:#x}")`.
pub fn format_in_base(n: u64, base: Base) -> String {
    todo!("lab-1-4-base-converter: implement me")
}

/// `args` excludes the program name: `["255", "hex"]`.
/// Errors are human-readable messages.
pub fn run(args: &[String]) -> Result<String, String> {
    todo!("lab-1-4-base-converter: implement me")
}
