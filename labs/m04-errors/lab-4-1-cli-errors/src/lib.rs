#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 4.1 — Refactor away panics and matches with `?`.
//!
//! This is Lab 1.4 again, but now `run` returns `Box<dyn Error>` and every
//! failure is propagated with `?`. Useful conversions:
//!   - `Option` → `Result`: `.ok_or("message")?` or `.ok_or_else(|| format!(..))?`
//!   - `&str` and `String` convert into `Box<dyn Error>` automatically via `?`
//!   - add context to a parse error: `.map_err(|e| format!("bad number '{s}': {e}"))?`
//!
//! Goal: no `match` on errors and no `unwrap()` anywhere in this file.

use std::error::Error;

pub fn format_in_base(n: u64, base: &str) -> Option<String> {
    todo!("lab-4-1-cli-errors: implement me")
}

/// `args` excludes the program name: `["255", "hex"]`.
pub fn run(args: &[String]) -> Result<String, Box<dyn Error>> {
    todo!("lab-4-1-cli-errors: implement me")
}
