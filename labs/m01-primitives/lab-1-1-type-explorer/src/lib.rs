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
    // SOLUTION-START
    match type_name {
        "i8" | "u8" | "bool" => 1,
        "i16" => 2,
        "i32" | "f32" | "char" => 4,
        "i64" | "isize" | "usize" | "f64" => 8,
        "i128" => 16,
        "()" => 0,
        // fat pointers: (pointer, length)
        "&str" | "&[u8]" => 16,
        // (pointer, capacity, length)
        "String" | "Vec<u8>" => 24,
        "[u8; 16]" => 16,
        // u8 + 3 bytes padding + u32 (alignment of the largest field)
        "(u8, u32)" => 8,
        "Box<i32>" => 8,
        // needs a separate tag byte: every u8 value is valid
        "Option<u8>" => 2,
        // tag padded to u32's alignment
        "Option<u32>" => 8,
        // niche optimisation: bool only uses 0 and 1, so None = 2
        "Option<bool>" => 1,
        // char is at most 0x10FFFF, plenty of invalid bit patterns for None
        "Option<char>" => 4,
        // references and Box are never null, so None = null pointer
        "Option<&u8>" | "Option<Box<i32>>" => 8,
        other => panic!("unknown type {other}"),
    }
    // SOLUTION-END
}
