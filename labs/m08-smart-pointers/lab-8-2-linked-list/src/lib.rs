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
        // SOLUTION-START
        let new = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new);
        self.len += 1;
        // SOLUTION-END
    }

    pub fn pop(&mut self) -> Option<T> {
        // SOLUTION-START
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.elem
        })
        // SOLUTION-END
    }

    pub fn peek(&self) -> Option<&T> {
        // SOLUTION-START
        self.head.as_ref().map(|n| &n.elem)
        // SOLUTION-END
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // SOLUTION-START
        self.head.as_mut().map(|n| &mut n.elem)
        // SOLUTION-END
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Reverse in place by relinking nodes — no allocation, no element moves.
    pub fn reverse(&mut self) {
        // SOLUTION-START
        let mut prev: Link<T> = None;
        let mut cur = self.head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        self.head = prev;
        // SOLUTION-END
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

// SOLUTION-START
//~ // TODO: the default (recursive) drop overflows the stack on a very long list
//~ // (see tests/drop_long.rs). Write `impl<T> Drop for List<T>` here that
//~ // unlinks the nodes in a loop instead.
/// The default (recursive) drop would overflow the stack on a very long list.
/// Drop iteratively instead.
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
        }
    }
}
// SOLUTION-END

/// Consuming iterator: `for x in list`.
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        // SOLUTION-START
        self.0.pop()
        // SOLUTION-END
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
        // SOLUTION-START
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
        // SOLUTION-END
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}
