use lab_8_2_linked_list::List;

#[test]
fn push_pop_peek() {
    let mut l = List::new();
    assert_eq!(l.pop(), None);
    l.push(1);
    l.push(2);
    l.push(3);
    assert_eq!(l.len(), 3);
    assert_eq!(l.peek(), Some(&3));
    if let Some(top) = l.peek_mut() {
        *top *= 10;
    }
    assert_eq!(l.pop(), Some(30));
    assert_eq!(l.pop(), Some(2));
    l.push(4);
    assert_eq!(l.pop(), Some(4));
    assert_eq!(l.pop(), Some(1));
    assert!(l.is_empty());
}

#[test]
fn iterators() {
    let mut l = List::new();
    for x in 1..=4 {
        l.push(x);
    }
    let borrowed: Vec<&i32> = l.iter().collect();
    assert_eq!(borrowed, [&4, &3, &2, &1]);
    let sum: i32 = (&l).into_iter().sum();
    assert_eq!(sum, 10);
    let owned: Vec<i32> = l.into_iter().collect();
    assert_eq!(owned, [4, 3, 2, 1]);
}

#[test]
fn reverse() {
    let mut l = List::new();
    for s in ["a", "b", "c"] {
        l.push(s.to_string());
    }
    l.reverse();
    assert_eq!(
        l.iter().map(String::as_str).collect::<Vec<_>>(),
        ["a", "b", "c"]
    );
    assert_eq!(l.len(), 3);
    let mut empty: List<i32> = List::new();
    empty.reverse();
    assert!(empty.is_empty());
}
