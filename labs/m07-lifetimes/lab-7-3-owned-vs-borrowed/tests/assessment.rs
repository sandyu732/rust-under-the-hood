use lab_7_3_owned_vs_borrowed::*;
use std::borrow::Cow;

#[test]
fn owned_version_is_self_contained() {
    let r = make_request_owned(42);
    assert_eq!(r.path, "/items/42");
    assert_eq!(
        r.headers,
        vec![("Host".to_string(), "localhost".to_string())]
    );
}

#[test]
fn borrowed_version_uses_callers_buffer() {
    let mut buf = String::new();
    let r = make_request_in(&mut buf, 7);
    assert_eq!(r.path, "/items/7");
    assert_eq!(r.header("host"), Some("localhost"));
    // `buf` can't be modified while `r` is alive — try adding `buf.clear();` above this line.
}

#[test]
fn cow_borrows_when_it_can() {
    assert!(matches!(normalize_path("/a/b"), Cow::Borrowed("/a/b")));
    assert!(matches!(normalize_path("/"), Cow::Borrowed("/")));
    assert_eq!(normalize_path("//a///b/"), "/a/b");
    assert!(matches!(normalize_path("/a/"), Cow::Owned(_)));
}
