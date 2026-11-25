use lab_7_1_longest::*;

#[test]
fn longest_picks_the_longer() {
    assert_eq!(longest("abc", "de"), "abc");
    assert_eq!(longest("ab", "cd"), "ab", "ties go to the first");
}

#[test]
fn prefix_result_outlives_the_delimiter() {
    let text = String::from("key=value");
    let key;
    {
        let delim = String::from("=");
        key = prefix_before(&text, &delim);
    } // delim dropped here — fine, because the result only borrows `text`
    assert_eq!(key, "key");
    assert_eq!(prefix_before("no-delim", "="), "no-delim");
}

#[test]
fn results_outlive_the_highlighter() {
    let text = String::from("the cat sat on the mat");
    let hits;
    {
        let h = Highlighter::new(&text);
        hits = h.find_all("at");
    } // h dropped — results borrow `text`, not `h`
    assert_eq!(hits, ["at", "at", "at"]);
    assert!(Highlighter::new(&text).find_all("").is_empty());
}

#[test]
fn first_line() {
    let h = Highlighter::new("one\ntwo");
    assert_eq!(h.first_line(), "one");
    assert_eq!(Highlighter::new("").first_line(), "");
}
