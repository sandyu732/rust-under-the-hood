//! Lab 5.3 — A generic stack, with conditional trait implementations.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        // SOLUTION-START
        Stack { items: Vec::new() }
        // SOLUTION-END
    }

    pub fn push(&mut self, item: T) {
        // SOLUTION-START
        self.items.push(item);
        // SOLUTION-END
    }

    pub fn pop(&mut self) -> Option<T> {
        // SOLUTION-START
        self.items.pop()
        // SOLUTION-END
    }

    pub fn peek(&self) -> Option<&T> {
        // SOLUTION-START
        self.items.last()
        // SOLUTION-END
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // SOLUTION-START
        self.items.last_mut()
        // SOLUTION-END
    }

    pub fn len(&self) -> usize {
        // SOLUTION-START
        self.items.len()
        // SOLUTION-END
    }

    pub fn is_empty(&self) -> bool {
        // SOLUTION-START
        self.items.is_empty()
        // SOLUTION-END
    }
}

/// Only stacks of orderable items get `max`.
impl<T: Ord> Stack<T> {
    pub fn max(&self) -> Option<&T> {
        // SOLUTION-START
        self.items.iter().max()
        // SOLUTION-END
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
        // SOLUTION-START
        write!(f, "[")?;
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{item}")?;
        }
        write!(f, "]")
        // SOLUTION-END
    }
}

/// Lets you write `let s: Stack<_> = (1..=3).collect();`
impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        // SOLUTION-START
        Stack {
            items: iter.into_iter().collect(),
        }
        // SOLUTION-END
    }
}
