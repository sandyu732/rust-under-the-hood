use lab_2_3_signatures::*;

#[test]
fn in_place() {
    let mut v = vec![1, -2, 3];
    double_in_place(&mut v);
    assert_eq!(v, [2, -4, 6]);
    let mut arr = [5, 6];
    double_in_place(&mut arr); // arrays coerce to slices too
    assert_eq!(arr, [10, 12]);
}

#[test]
fn borrowed_input_is_untouched() {
    let v = vec![1, 2, 3];
    assert_eq!(doubled(&v), [2, 4, 6]);
    assert_eq!(v, [1, 2, 3]);
}

#[test]
fn owned_version_reuses_the_allocation() {
    let v = vec![1, 2, 3, 4];
    let ptr = v.as_ptr();
    let w = into_doubled(v);
    assert_eq!(w, [2, 4, 6, 8]);
    assert_eq!(
        w.as_ptr(),
        ptr,
        "into_doubled should reuse the input buffer"
    );
}

#[test]
fn append_reuses_buffer() {
    let mut s = String::with_capacity(32);
    s.push_str("hello");
    let ptr = s.as_ptr();
    let out = append_owned(s, " world");
    assert_eq!(out, "hello world");
    assert_eq!(out.as_ptr(), ptr);
}

#[test]
fn total_len_works() {
    let words = vec!["ab".to_string(), "cde".to_string()];
    assert_eq!(total_len(&words), 5);
    assert_eq!(total_len(&[]), 0);
}
