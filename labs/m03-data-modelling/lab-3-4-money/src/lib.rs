//! Lab 3.4 — Newtype money in a module with private fields.
//!
//! Everything lives in `src/bank.rs`. Code outside that module (like the
//! assessment) can only use the public API — it can't read or set `cents`
//! directly. Try it: add `let m = Money { cents: 1, currency: Currency::Usd };`
//! to a test and read the error.

pub mod bank;
