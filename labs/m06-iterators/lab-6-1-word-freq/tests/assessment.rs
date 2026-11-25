use lab_6_1_word_freq::*;

#[test]
fn splits_and_normalises() {
    let w: Vec<String> = words("Hello, world! Don't stop -- 'tis HELLO.").collect();
    assert_eq!(w, ["hello", "world", "don't", "stop", "tis", "hello"]);
}

#[test]
fn counts() {
    let f = word_frequencies("a b a C c c");
    assert_eq!(f["a"], 2);
    assert_eq!(f["c"], 3);
    assert_eq!(f.get("d"), None);
}

#[test]
fn top_n_orders_by_count_then_word() {
    let f = word_frequencies("b a c b a d b");
    assert_eq!(top_n(&f, 3), vec![("b", 3), ("a", 2), ("c", 1)]);
    assert_eq!(top_n(&f, 100).len(), 4);
}

#[test]
fn no_index_loops() {
    let src = include_str!("../src/lib.rs");
    let code: String = src
        .lines()
        .filter(|l| !l.trim_start().starts_with("//"))
        .collect();
    assert!(
        !code.contains("for i in 0.."),
        "use iterators instead of index loops"
    );
}
