//! Lab 8.3 — A tree with parent pointers: `Rc`, `Weak` and `RefCell`.
//!
//! Children are owned (`Rc`); the parent link is NOT owned (`Weak`),
//! otherwise parent ↔ child would form a reference cycle and never be freed.
//! `RefCell` lets us mutate `children`/`parent` through a shared `Rc`.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub children: RefCell<Vec<Rc<Node>>>,
    pub parent: RefCell<Weak<Node>>,
}

impl Node {
    pub fn new(value: i32) -> Rc<Node> {
        // SOLUTION-START
        Rc::new(Node {
            value,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
        })
        // SOLUTION-END
    }
}

/// Attach `child` under `parent` and set the child's parent link.
/// Hint: `Rc::downgrade(parent)`.
pub fn add_child(parent: &Rc<Node>, child: Rc<Node>) {
    // SOLUTION-START
    *child.parent.borrow_mut() = Rc::downgrade(parent);
    parent.children.borrow_mut().push(child);
    // SOLUTION-END
}

/// Values from `node` up to the root, e.g. `[leaf, mid, root]`.
/// Hint: `weak.upgrade()` returns `Option<Rc<Node>>`.
pub fn path_to_root(node: &Rc<Node>) -> Vec<i32> {
    // SOLUTION-START
    let mut path = vec![node.value];
    let mut current = node.parent.borrow().upgrade();
    while let Some(n) = current {
        path.push(n.value);
        current = n.parent.borrow().upgrade();
    }
    path
    // SOLUTION-END
}

/// Sum of this node's value and all its descendants.
pub fn sum_subtree(node: &Node) -> i32 {
    // SOLUTION-START
    node.value
        + node
            .children
            .borrow()
            .iter()
            .map(|c| sum_subtree(c))
            .sum::<i32>()
    // SOLUTION-END
}

/// Number of edges from `node` to the root (root has depth 0).
pub fn depth(node: &Rc<Node>) -> usize {
    // SOLUTION-START
    path_to_root(node).len() - 1
    // SOLUTION-END
}
