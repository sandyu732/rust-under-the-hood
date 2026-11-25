#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 5.4 — Custom ordering and top-K with a BinaryHeap.

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Employee {
    pub name: String,
    pub dept: String,
    pub salary: u32,
}

impl Employee {
    pub fn new(name: &str, dept: &str, salary: u32) -> Self {
        Employee {
            name: name.into(),
            dept: dept.into(),
            salary,
        }
    }
}

/// Order by department ascending, then salary DESCENDING, then name ascending.
/// Hint: `a.cmp(&b).then_with(|| ...)` and `.reverse()`.
impl Ord for Employee {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!("lab-5-4-top-k: implement me")
    }
}

impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Sort using the `Ord` impl above.
pub fn sorted(mut staff: Vec<Employee>) -> Vec<Employee> {
    todo!("lab-5-4-top-k: implement me")
}

/// The `k` highest earners, highest first; ties broken by name ascending.
/// Use a `BinaryHeap` of size at most `k` (a min-heap via `Reverse`), so the
/// work is O(n log k) rather than sorting everything.
pub fn top_k_earners(staff: &[Employee], k: usize) -> Vec<&Employee> {
    todo!("lab-5-4-top-k: implement me")
}
