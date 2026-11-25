use lab_5_3_stack::Stack;

#[derive(Debug, Clone, PartialEq)]
struct Job {
    id: u32,
}

#[test]
fn lifo_with_ints() {
    let mut s = Stack::new();
    assert!(s.is_empty());
    s.push(1);
    s.push(2);
    assert_eq!(s.peek(), Some(&2));
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.len(), 1);
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
}

#[test]
fn works_with_strings_and_custom_types() {
    let mut s: Stack<String> = Stack::default();
    s.push("a".into());
    if let Some(top) = s.peek_mut() {
        top.push('!');
    }
    assert_eq!(s.pop().as_deref(), Some("a!"));

    let mut jobs = Stack::new();
    jobs.push(Job { id: 7 });
    assert_eq!(jobs.peek().map(|j| j.id), Some(7));
}

#[test]
fn display_and_collect() {
    let s: Stack<i32> = (1..=3).collect();
    assert_eq!(s.to_string(), "[1, 2, 3]");
    assert_eq!(Stack::<i32>::new().to_string(), "[]");
}

#[test]
fn max_for_ord_types() {
    let s: Stack<&str> = ["pear", "apple", "zucchini"].into_iter().collect();
    assert_eq!(s.max(), Some(&"zucchini"));
}
