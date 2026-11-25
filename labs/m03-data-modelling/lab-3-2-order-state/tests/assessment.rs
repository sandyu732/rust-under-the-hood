use lab_3_2_order_state::*;

fn item(sku: &str, qty: u32, cents: u64) -> Item {
    Item {
        sku: sku.into(),
        qty,
        unit_price_cents: cents,
    }
}

fn submitted() -> Order {
    Order::new()
        .add_item(item("A", 2, 500))
        .and_then(|o| o.add_item(item("B", 1, 250)))
        .and_then(|o| o.submit("sam"))
        .expect("draft with items should submit")
}

#[test]
fn happy_path() {
    let o = submitted();
    assert_eq!(o.status(), "submitted");
    assert_eq!(o.total_cents(), 1250);
    let o = o.approve("lee").unwrap().ship("TRACK-1").unwrap();
    assert_eq!(o.status(), "shipped");
    assert_eq!(o.items().len(), 2);
    match o {
        Order::Shipped { tracking, .. } => assert_eq!(tracking, "TRACK-1"),
        other => panic!("expected shipped, got {other:?}"),
    }
}

#[test]
fn empty_draft_cannot_submit() {
    assert_eq!(Order::new().submit("sam"), Err(OrderError::EmptyOrder));
}

#[test]
fn rejected_orders_cannot_ship() {
    let rejected = submitted().reject("out of stock").unwrap();
    assert_eq!(rejected.status(), "rejected");
    assert_eq!(
        rejected.ship("X"),
        Err(OrderError::InvalidTransition {
            from: "rejected",
            action: "ship"
        })
    );
}

#[test]
fn invalid_transitions_name_the_state() {
    assert_eq!(
        Order::new().approve("lee"),
        Err(OrderError::InvalidTransition {
            from: "draft",
            action: "approve"
        })
    );
    assert_eq!(
        submitted().add_item(item("C", 1, 1)),
        Err(OrderError::InvalidTransition {
            from: "submitted",
            action: "add_item"
        })
    );
    let shipped = submitted().approve("lee").unwrap().ship("T").unwrap();
    assert_eq!(
        shipped.reject("late"),
        Err(OrderError::InvalidTransition {
            from: "shipped",
            action: "reject"
        })
    );
}
