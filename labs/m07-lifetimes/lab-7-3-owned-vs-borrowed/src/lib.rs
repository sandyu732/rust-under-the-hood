//! Lab 7.3 — The function that can't exist (and two ways around it).
//!
//! `cargo build -p lab-7-3-owned-vs-borrowed --features broken` shows why you
//! can't build a `String` inside a function and return a `Request` borrowing it.
//! Fix 1: return the owned type. Fix 2: let the CALLER own the buffer.
//! Bonus: `Cow` — borrow when you can, allocate only when you must.

use lab_7_2_http_parser::{OwnedRequest, Request, parse_request};
use std::borrow::Cow;

#[cfg(feature = "broken")]
pub mod broken;

/// Builds `GET /items/<id> HTTP/1.1` with a `Host: localhost` header and empty body.
pub fn raw_request(id: u32) -> String {
    format!("GET /items/{id} HTTP/1.1\r\nHost: localhost\r\n\r\n")
}

/// Fix 1: build the raw text, parse it, and return an OWNED copy.
pub fn make_request_owned(id: u32) -> OwnedRequest {
    // SOLUTION-START
    let raw = raw_request(id);
    parse_request(&raw)
        .expect("raw_request always produces a valid request")
        .into()
    // SOLUTION-END
}

/// Fix 2: write the raw text into the caller's buffer and return a request
/// that borrows from it. The lifetime `'a` ties the result to `buf`.
pub fn make_request_in(buf: &mut String, id: u32) -> Request<'_> {
    // SOLUTION-START
    *buf = raw_request(id);
    parse_request(buf).expect("raw_request always produces a valid request")
    // SOLUTION-END
}

/// Collapse repeated slashes and drop a trailing slash (except for "/").
/// Return `Cow::Borrowed` when the input is already normal (no allocation),
/// `Cow::Owned` only when something changed.
pub fn normalize_path(path: &str) -> Cow<'_, str> {
    // SOLUTION-START
    let needs_work = path.contains("//") || (path.len() > 1 && path.ends_with('/'));
    if !needs_work {
        return Cow::Borrowed(path);
    }
    let mut out = String::with_capacity(path.len());
    for c in path.chars() {
        if !(c == '/' && out.ends_with('/')) {
            out.push(c);
        }
    }
    if out.len() > 1 && out.ends_with('/') {
        out.pop();
    }
    Cow::Owned(out)
    // SOLUTION-END
}
