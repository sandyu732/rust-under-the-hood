#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 8.2 — A singly linked stack with `Option<Box<Node<T>>>`.
//!
//! Companion reading: "Learning Rust With Entirely Too Many Linked Lists",
//! chapters 2–3 (https://rust-unofficial.github.io/too-many-lists/).
//! Your best friends: `Option::take()`, `Option::map()`, `as_deref()`, `as_deref_mut()`.

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
    len: usize,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, len: 0 }
    }

    pub fn push(&mut self, elem: T) {
        todo!("lab-8-2-linked-list: implement me")
    }

    pub fn pop(&mut self) -> Option<T> {
        todo!("lab-8-2-linked-list: implement me")
    }

    pub fn peek(&self) -> Option<&T> {
        todo!("lab-8-2-linked-list: implement me")
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        todo!("lab-8-2-linked-list: implement me")
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Reverse in place by relinking nodes — no allocation, no element moves.
    pub fn reverse(&mut self) {
        todo!("lab-8-2-linked-list: implement me")
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: the default (recursive) drop overflows the stack on a very long list
// (see tests/drop_long.rs). Write `impl<T> Drop for List<T>` here that
// unlinks the nodes in a loop instead.

/// Consuming iterator: `for x in list`.
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        todo!("lab-8-2-linked-list: implement me")
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

/// Borrowing iterator: `for x in &list` / `list.iter()`.
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        todo!("lab-8-2-linked-list: implement me")
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}
