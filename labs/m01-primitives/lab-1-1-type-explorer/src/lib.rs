#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 1.1 — Type explorer.
//!
//! Predict the size in bytes of each type (assume a 64-bit machine) by filling
//! in the `match` below. Do NOT call `std::mem::size_of` here — that's what the
//! assessment does to check you. Run `cargo run -p lab-1-1-type-explorer` to
//! see a table of your predictions vs reality.

/// Every type the assessment checks, in the order the table prints them.
pub const TYPES: &[&str] = &[
    "i8",
    "i16",
    "i32",
    "i64",
    "i128",
    "isize",
    "u8",
    "usize",
    "f32",
    "f64",
    "bool",
    "char",
    "()",
    "&str",
    "String",
    "Vec<u8>",
    "[u8; 16]",
    "(u8, u32)",
    "&[u8]",
    "Box<i32>",
    "Option<u8>",
    "Option<u32>",
    "Option<bool>",
    "Option<char>",
    "Option<&u8>",
    "Option<Box<i32>>",
];

/// Your predicted size, in bytes, of the type named `type_name`.
pub fn predicted_size(type_name: &str) -> usize {
    todo!("lab-1-1-type-explorer: implement me")
}
