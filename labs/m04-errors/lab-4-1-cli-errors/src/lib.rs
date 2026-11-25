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
    // SOLUTION-START
    match base.to_ascii_lowercase().as_str() {
        "bin" => Some(format!("{n:#b}")),
        "oct" => Some(format!("{n:#o}")),
        "hex" => Some(format!("{n:#x}")),
        _ => None,
    }
    // SOLUTION-END
}

/// `args` excludes the program name: `["255", "hex"]`.
pub fn run(args: &[String]) -> Result<String, Box<dyn Error>> {
    // SOLUTION-START
    let number = args.first().ok_or("missing <number> argument")?;
    let base = args.get(1).ok_or("missing <bin|oct|hex> argument")?;
    if args.len() > 2 {
        return Err("too many arguments".into());
    }
    let n: u64 = number
        .parse()
        .map_err(|e| format!("'{number}' is not a valid number: {e}"))?;
    let out = format_in_base(n, base).ok_or_else(|| format!("unknown base '{base}'"))?;
    Ok(out)
    // SOLUTION-END
}
