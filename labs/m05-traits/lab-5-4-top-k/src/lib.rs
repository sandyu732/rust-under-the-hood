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
        // SOLUTION-START
        self.dept
            .cmp(&other.dept)
            .then_with(|| self.salary.cmp(&other.salary).reverse())
            .then_with(|| self.name.cmp(&other.name))
        // SOLUTION-END
    }
}

impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Sort using the `Ord` impl above.
pub fn sorted(mut staff: Vec<Employee>) -> Vec<Employee> {
    // SOLUTION-START
    staff.sort();
    staff
    // SOLUTION-END
}

/// The `k` highest earners, highest first; ties broken by name ascending.
/// Use a `BinaryHeap` of size at most `k` (a min-heap via `Reverse`), so the
/// work is O(n log k) rather than sorting everything.
pub fn top_k_earners(staff: &[Employee], k: usize) -> Vec<&Employee> {
    // SOLUTION-START
    if k == 0 {
        return Vec::new();
    }
    // Key: higher salary is better; for equal salary, smaller name is better.
    let mut heap = BinaryHeap::with_capacity(k + 1);
    for e in staff {
        heap.push(Reverse((e.salary, Reverse(e.name.as_str()), e)));
        if heap.len() > k {
            heap.pop(); // removes the current worst
        }
    }
    let mut best: Vec<_> = heap.into_iter().map(|Reverse(entry)| entry).collect();
    best.sort_by(|a, b| b.cmp(a));
    best.into_iter().map(|(_, _, e)| e).collect()
    // SOLUTION-END
}
