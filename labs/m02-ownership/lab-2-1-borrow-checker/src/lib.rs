//! Lab 2.1 — Break the borrow checker (the most important lab in this path).
//!
//! 1. Read `src/broken.rs`. Predict which line each function fails on.
//! 2. Run `cargo build -p lab-2-1-borrow-checker --features broken` and read
//!    every error to the end. Run `rustc --explain <CODE>` for each code.
//! 3. Fill in `NOTES.md` (error code + one-sentence explanation + your fix).
//! 4. Implement the fixed versions in `src/fixed.rs` so the assessment passes.
//!    Keep each fixed function as close to the broken one as you can.

#[cfg(feature = "broken")]
pub mod broken;
pub mod fixed;
