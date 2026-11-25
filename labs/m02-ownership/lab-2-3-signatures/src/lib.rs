#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 2.3 — Three signatures, one job.
//!
//! The *signature* tells the caller who owns what:
//!   `&mut [T]` → I'll modify yours in place
//!   `&[T]`     → I'll read yours and give you something new
//!   `Vec<T>`   → give it to me; I may reuse its allocation

/// Double every element in place. No allocation.
pub fn double_in_place(v: &mut [i32]) {
    todo!("lab-2-3-signatures: implement me")
}

/// Return a new Vec with every element doubled; the input is untouched.
pub fn doubled(v: &[i32]) -> Vec<i32> {
    todo!("lab-2-3-signatures: implement me")
}

/// Take ownership and return the doubled Vec, REUSING the same heap buffer
/// (the assessment checks the pointer). Hint: you already wrote a helper.
pub fn into_doubled(v: Vec<i32>) -> Vec<i32> {
    todo!("lab-2-3-signatures: implement me")
}

/// Append `suffix` to `base`, reusing `base`'s buffer.
pub fn append_owned(base: String, suffix: &str) -> String {
    todo!("lab-2-3-signatures: implement me")
}

/// Total byte length of all strings. Note the parameter is `&[String]`, so
/// it accepts `&Vec<String>` and arrays alike.
pub fn total_len(words: &[String]) -> usize {
    todo!("lab-2-3-signatures: implement me")
}
