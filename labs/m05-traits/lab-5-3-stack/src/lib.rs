#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 5.3 — A generic stack, with conditional trait implementations.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        todo!("lab-5-3-stack: implement me")
    }

    pub fn push(&mut self, item: T) {
        todo!("lab-5-3-stack: implement me")
    }

    pub fn pop(&mut self) -> Option<T> {
        todo!("lab-5-3-stack: implement me")
    }

    pub fn peek(&self) -> Option<&T> {
        todo!("lab-5-3-stack: implement me")
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        todo!("lab-5-3-stack: implement me")
    }

    pub fn len(&self) -> usize {
        todo!("lab-5-3-stack: implement me")
    }

    pub fn is_empty(&self) -> bool {
        todo!("lab-5-3-stack: implement me")
    }
}

/// Only stacks of orderable items get `max`.
impl<T: Ord> Stack<T> {
    pub fn max(&self) -> Option<&T> {
        todo!("lab-5-3-stack: implement me")
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Bottom to top: `[1, 2, 3]`. Only exists when `T: Display`.
impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("lab-5-3-stack: implement me")
    }
}

/// Lets you write `let s: Stack<_> = (1..=3).collect();`
impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        todo!("lab-5-3-stack: implement me")
    }
}
