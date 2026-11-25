use lab_2_1_borrow_checker::fixed::*;

#[test]
fn a() {
    assert_eq!(a_use_after_move(), 4);
}

#[test]
fn b() {
    assert_eq!(b_two_mutable_borrows(), vec![1, 2, 3, 4, 5]);
}

#[test]
fn c() {
    assert_eq!(c_push_while_borrowed(), (10, 4));
}

#[test]
fn d() {
    assert_eq!(d_dangling(), "hello");
}

#[test]
fn e() {
    let mut v = vec![1, 2, 3, 4];
    e_mutate_while_iterating(&mut v);
    assert_eq!(v, vec![1, 2, 3, 4, 20, 40]);
    let mut empty: Vec<i32> = vec![];
    e_mutate_while_iterating(&mut empty);
    assert!(empty.is_empty());
}
