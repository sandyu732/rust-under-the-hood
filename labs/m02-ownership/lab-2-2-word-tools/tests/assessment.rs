use lab_2_2_word_tools::*;

/// True if `part` is a slice of `whole`'s memory (i.e. no copy was made).
fn points_into(whole: &str, part: &str) -> bool {
    let w = whole.as_bytes().as_ptr_range();
    let p = part.as_bytes().as_ptr_range();
    part.is_empty() || (w.start <= p.start && p.end <= w.end)
}

#[test]
fn first_word_works_with_any_string_kind() {
    let owned = String::from("hello world");
    assert_eq!(first_word(&owned), "hello");
    assert_eq!(first_word("  leading spaces"), "leading");
    assert_eq!(first_word(&owned[6..]), "world");
    assert_eq!(first_word(""), "");
    assert!(points_into(&owned, first_word(&owned)));
}

#[test]
fn longest_word_first_wins_on_tie() {
    let s = String::from("the quick brown fox jumped");
    assert_eq!(longest_word(&s), "jumped");
    assert_eq!(longest_word("aa bb cc"), "aa");
    assert_eq!(longest_word("   "), "");
    assert!(points_into(&s, longest_word(&s)));
}

#[test]
fn counts_words() {
    assert_eq!(word_count("one two  three\nfour\tfive"), 5);
    assert_eq!(word_count(""), 0);
}

#[test]
fn trims_punctuation_without_copying() {
    let s = String::from("(hello!)");
    assert_eq!(trim_punctuation(&s), "hello");
    assert_eq!(trim_punctuation("..."), "");
    assert_eq!(trim_punctuation("don't"), "don't");
    assert!(points_into(&s, trim_punctuation(&s)));
}
