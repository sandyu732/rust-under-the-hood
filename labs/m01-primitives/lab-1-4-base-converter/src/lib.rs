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
    // SOLUTION-START
    match s.to_ascii_lowercase().as_str() {
        "bin" => Some(Base::Bin),
        "oct" => Some(Base::Oct),
        "hex" => Some(Base::Hex),
        _ => None,
    }
    // SOLUTION-END
}

/// 255, Hex → "0xff"; 5, Bin → "0b101"; 8, Oct → "0o10".
/// Hint: `format!("{n:#x}")`.
pub fn format_in_base(n: u64, base: Base) -> String {
    // SOLUTION-START
    match base {
        Base::Bin => format!("{n:#b}"),
        Base::Oct => format!("{n:#o}"),
        Base::Hex => format!("{n:#x}"),
    }
    // SOLUTION-END
}

/// `args` excludes the program name: `["255", "hex"]`.
/// Errors are human-readable messages.
pub fn run(args: &[String]) -> Result<String, String> {
    // SOLUTION-START
    if args.len() != 2 {
        return Err("usage: base-converter <number> <bin|oct|hex>".to_string());
    }
    let n = match args[0].parse::<u64>() {
        Ok(n) => n,
        Err(e) => return Err(format!("'{}' is not a valid number: {e}", args[0])),
    };
    match parse_base(&args[1]) {
        Some(base) => Ok(format_in_base(n, base)),
        None => Err(format!(
            "unknown base '{}', expected bin, oct or hex",
            args[1]
        )),
    }
    // SOLUTION-END
}
