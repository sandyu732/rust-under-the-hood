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
        // SOLUTION-START
        Order::Draft { items: Vec::new() }
        // SOLUTION-END
    }

    /// "draft" | "submitted" | "approved" | "rejected" | "shipped"
    pub fn status(&self) -> &'static str {
        // SOLUTION-START
        match self {
            Order::Draft { .. } => "draft",
            Order::Submitted { .. } => "submitted",
            Order::Approved { .. } => "approved",
            Order::Rejected { .. } => "rejected",
            Order::Shipped { .. } => "shipped",
        }
        // SOLUTION-END
    }

    pub fn items(&self) -> &[Item] {
        // SOLUTION-START
        match self {
            Order::Draft { items }
            | Order::Submitted { items, .. }
            | Order::Approved { items, .. }
            | Order::Rejected { items, .. }
            | Order::Shipped { items, .. } => items,
        }
        // SOLUTION-END
    }

    pub fn total_cents(&self) -> u64 {
        // SOLUTION-START
        self.items()
            .iter()
            .map(|i| i.qty as u64 * i.unit_price_cents)
            .sum()
        // SOLUTION-END
    }

    /// Only drafts accept items.
    pub fn add_item(self, item: Item) -> Result<Order, OrderError> {
        // SOLUTION-START
        match self {
            Order::Draft { mut items } => {
                items.push(item);
                Ok(Order::Draft { items })
            }
            other => Err(other.invalid("add_item")),
        }
        // SOLUTION-END
    }

    /// Draft → Submitted. Fails with `EmptyOrder` if there are no items.
    pub fn submit(self, by: &str) -> Result<Order, OrderError> {
        // SOLUTION-START
        match self {
            Order::Draft { items } if items.is_empty() => Err(OrderError::EmptyOrder),
            Order::Draft { items } => Ok(Order::Submitted {
                items,
                submitted_by: by.to_string(),
            }),
            other => Err(other.invalid("submit")),
        }
        // SOLUTION-END
    }

    /// Submitted → Approved.
    pub fn approve(self, by: &str) -> Result<Order, OrderError> {
        // SOLUTION-START
        match self {
            Order::Submitted { items, .. } => Ok(Order::Approved {
                items,
                approved_by: by.to_string(),
            }),
            other => Err(other.invalid("approve")),
        }
        // SOLUTION-END
    }

    /// Submitted → Rejected.
    pub fn reject(self, reason: &str) -> Result<Order, OrderError> {
        // SOLUTION-START
        match self {
            Order::Submitted { items, .. } => Ok(Order::Rejected {
                items,
                reason: reason.to_string(),
            }),
            other => Err(other.invalid("reject")),
        }
        // SOLUTION-END
    }

    /// Approved → Shipped.
    pub fn ship(self, tracking: &str) -> Result<Order, OrderError> {
        // SOLUTION-START
        match self {
            Order::Approved { items, .. } => Ok(Order::Shipped {
                items,
                tracking: tracking.to_string(),
            }),
            other => Err(other.invalid("ship")),
        }
        // SOLUTION-END
    }

    // SOLUTION-ONLY-START
    fn invalid(&self, action: &'static str) -> OrderError {
        OrderError::InvalidTransition {
            from: self.status(),
            action,
        }
    }
    // SOLUTION-ONLY-END
}

impl Default for Order {
    fn default() -> Self {
        Self::new()
    }
}
