use lab_8_4_arena_tree::*;

/// Same shape as Lab 8.3:  root(1) ─┬─ a(2) ── leaf(4)
///                                  └─ b(3)
fn build() -> (Tree, NodeId, NodeId) {
    let mut t = Tree::with_root(1);
    let root = t.root();
    let a = t.add_child(root, 2);
    let _b = t.add_child(root, 3);
    let leaf = t.add_child(a, 4);
    (t, a, leaf)
}

#[test]
fn same_answers_as_the_rc_version() {
    let (t, a, leaf) = build();
    assert_eq!(t.path_to_root(leaf), [4, 2, 1]);
    assert_eq!(t.depth(leaf), 2);
    assert_eq!(t.sum_subtree(t.root()), 10);
    assert_eq!(t.sum_subtree(a), 6);
    assert_eq!(t.len(), 4);
}

#[test]
fn navigation_and_mutation_without_refcell() {
    let (mut t, a, leaf) = build();
    assert_eq!(t.parent(leaf), Some(a));
    assert_eq!(t.parent(t.root()), None);
    assert_eq!(t.children(t.root()).len(), 2);
    t.set_value(leaf, 40); // plain &mut self — no runtime borrow checks
    assert_eq!(t.sum_subtree(t.root()), 46);
}
