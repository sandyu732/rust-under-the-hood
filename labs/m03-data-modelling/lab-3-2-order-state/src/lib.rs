#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 3.2 — Type-safe order state machine.
//!
//!   Draft ──submit──▶ Submitted ──approve──▶ Approved ──ship──▶ Shipped
//!                         └────────reject──▶ Rejected
//!
//! Each variant holds only the data valid in that state (a Draft has no
//! tracking number; only Shipped does). Transitions take `self` by value, so
//! the old state is consumed and can't be reused by accident.

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub sku: String,
    pub qty: u32,
    pub unit_price_cents: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Order {
    Draft {
        items: Vec<Item>,
    },
    Submitted {
        items: Vec<Item>,
        submitted_by: String,
    },
    Approved {
        items: Vec<Item>,
        approved_by: String,
    },
    Rejected {
        items: Vec<Item>,
        reason: String,
    },
    Shipped {
        items: Vec<Item>,
        tracking: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderError {
    /// e.g. `{ from: "shipped", action: "approve" }`
    InvalidTransition {
        from: &'static str,
        action: &'static str,
    },
    /// Can't submit a draft with no items.
    EmptyOrder,
}

impl Order {
    /// An empty draft.
    pub fn new() -> Self {
        todo!("lab-3-2-order-state: implement me")
    }

    /// "draft" | "submitted" | "approved" | "rejected" | "shipped"
    pub fn status(&self) -> &'static str {
        todo!("lab-3-2-order-state: implement me")
    }

    pub fn items(&self) -> &[Item] {
        todo!("lab-3-2-order-state: implement me")
    }

    pub fn total_cents(&self) -> u64 {
        todo!("lab-3-2-order-state: implement me")
    }

    /// Only drafts accept items.
    pub fn add_item(self, item: Item) -> Result<Order, OrderError> {
        todo!("lab-3-2-order-state: implement me")
    }

    /// Draft → Submitted. Fails with `EmptyOrder` if there are no items.
    pub fn submit(self, by: &str) -> Result<Order, OrderError> {
        todo!("lab-3-2-order-state: implement me")
    }

    /// Submitted → Approved.
    pub fn approve(self, by: &str) -> Result<Order, OrderError> {
        todo!("lab-3-2-order-state: implement me")
    }

    /// Submitted → Rejected.
    pub fn reject(self, reason: &str) -> Result<Order, OrderError> {
        todo!("lab-3-2-order-state: implement me")
    }

    /// Approved → Shipped.
    pub fn ship(self, tracking: &str) -> Result<Order, OrderError> {
        todo!("lab-3-2-order-state: implement me")
    }
}

impl Default for Order {
    fn default() -> Self {
        Self::new()
    }
}
