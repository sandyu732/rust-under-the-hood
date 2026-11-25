#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 1.2 — Overflow lab.
//!
//! Rust makes integer overflow a *decision*. Implement each strategy using the
//! standard library methods on `u8` (look them up with `cargo doc --open` or
//! https://doc.rust-lang.org/std/primitive.u8.html).

use std::num::TryFromIntError;

/// Plain `+`. Panics on overflow in debug builds, wraps in release builds.
/// (Given — run `cargo run -p lab-1-2-overflow -- 250 10` with and without `--release`.)
pub fn plain_add(a: u8, b: u8) -> u8 {
    a + b
}

/// `None` on overflow.
pub fn checked(a: u8, b: u8) -> Option<u8> {
    todo!("lab-1-2-overflow: implement me")
}

/// Wraps around modulo 256.
pub fn wrapping(a: u8, b: u8) -> u8 {
    todo!("lab-1-2-overflow: implement me")
}

/// Clamps at `u8::MAX`.
pub fn saturating(a: u8, b: u8) -> u8 {
    todo!("lab-1-2-overflow: implement me")
}

/// Wrapped result plus a flag saying whether overflow happened.
pub fn overflowing(a: u8, b: u8) -> (u8, bool) {
    todo!("lab-1-2-overflow: implement me")
}

/// Lossy cast with `as` (keeps the low 8 bits).
pub fn to_u8_lossy(n: i32) -> u8 {
    todo!("lab-1-2-overflow: implement me")
}

/// Checked conversion with `TryFrom`.
pub fn to_u8_checked(n: i32) -> Result<u8, TryFromIntError> {
    todo!("lab-1-2-overflow: implement me")
}
