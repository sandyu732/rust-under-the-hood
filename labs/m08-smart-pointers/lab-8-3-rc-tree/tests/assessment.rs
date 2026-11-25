use lab_8_3_rc_tree::*;
use std::rc::Rc;

/// root(1) ─┬─ a(2) ── leaf(4)
///          └─ b(3)
fn build() -> (Rc<Node>, Rc<Node>, Rc<Node>) {
    let root = Node::new(1);
    let a = Node::new(2);
    let b = Node::new(3);
    let leaf = Node::new(4);
    add_child(&root, a.clone());
    add_child(&root, b);
    add_child(&a, leaf.clone());
    (root, a, leaf)
}

#[test]
fn walks_up_and_sums_down() {
    let (root, a, leaf) = build();
    assert_eq!(path_to_root(&leaf), [4, 2, 1]);
    assert_eq!(depth(&leaf), 2);
    assert_eq!(depth(&root), 0);
    assert_eq!(sum_subtree(&root), 10);
    assert_eq!(sum_subtree(&a), 6);
}

#[test]
fn reference_counts() {
    let (root, a, _leaf) = build();
    // `a` is owned by our local variable AND by root's children vec.
    assert_eq!(Rc::strong_count(&a), 2);
    // root is only strongly owned by our variable; children point to it weakly.
    assert_eq!(Rc::strong_count(&root), 1);
    assert_eq!(Rc::weak_count(&root), 2);
}

#[test]
fn weak_parent_does_not_keep_root_alive() {
    let (root, _a, leaf) = build();
    drop(root);
    // The root's only strong owner is gone, so it was freed — no cycle.
    assert_eq!(path_to_root(&leaf), [4, 2]);
}

#[test]
#[should_panic(expected = "already borrowed")]
fn refcell_enforces_borrow_rules_at_runtime() {
    let root = Node::new(1);
    let _first = root.children.borrow_mut();
    let _second = root.children.borrow_mut(); // panics: two &mut at runtime
}
