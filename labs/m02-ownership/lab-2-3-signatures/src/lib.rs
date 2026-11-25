//! Lab 2.3 — Three signatures, one job.
//!
//! The *signature* tells the caller who owns what:
//!   `&mut [T]` → I'll modify yours in place
//!   `&[T]`     → I'll read yours and give you something new
//!   `Vec<T>`   → give it to me; I may reuse its allocation

/// Double every element in place. No allocation.
pub fn double_in_place(v: &mut [i32]) {
    // SOLUTION-START
    for x in v.iter_mut() {
        *x *= 2;
    }
    // SOLUTION-END
}

/// Return a new Vec with every element doubled; the input is untouched.
pub fn doubled(v: &[i32]) -> Vec<i32> {
    // SOLUTION-START
    v.iter().map(|x| x * 2).collect()
    // SOLUTION-END
}

/// Take ownership and return the doubled Vec, REUSING the same heap buffer
/// (the assessment checks the pointer). Hint: you already wrote a helper.
pub fn into_doubled(v: Vec<i32>) -> Vec<i32> {
    // SOLUTION-START
    let mut v = v;
    double_in_place(&mut v);
    v
    // SOLUTION-END
}

/// Append `suffix` to `base`, reusing `base`'s buffer.
pub fn append_owned(base: String, suffix: &str) -> String {
    // SOLUTION-START
    let mut base = base;
    base.push_str(suffix);
    base
    // SOLUTION-END
}

/// Total byte length of all strings. Note the parameter is `&[String]`, so
/// it accepts `&Vec<String>` and arrays alike.
pub fn total_len(words: &[String]) -> usize {
    // SOLUTION-START
    words.iter().map(String::len).sum()
    // SOLUTION-END
}
