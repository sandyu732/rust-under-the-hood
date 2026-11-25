use lab_5_1_notifiers::*;

#[test]
fn each_notifier_behaves() {
    let mut c = Console::default();
    c.send("hi").unwrap();
    assert_eq!(c.sent, ["hi"]);

    let mut e = Email {
        to: "a@b.c".into(),
        outbox: vec![],
    };
    e.send("hello").unwrap();
    assert_eq!(e.outbox, ["To: a@b.c\nhello"]);
    let mut bad = Email {
        to: "nobody".into(),
        outbox: vec![],
    };
    assert_eq!(
        bad.send("x"),
        Err(NotifyError::InvalidTarget("nobody".into()))
    );

    let mut w = Webhook {
        url: "https://hook".into(),
        delivered: vec![],
    };
    assert_eq!(
        w.send(""),
        Err(NotifyError::Rejected("empty message".into()))
    );
    let mut insecure = Webhook {
        url: "http://hook".into(),
        delivered: vec![],
    };
    assert!(insecure.send("x").is_err());
}

#[test]
fn default_method_stops_at_first_error() {
    let mut w = Webhook {
        url: "https://hook".into(),
        delivered: vec![],
    };
    assert!(w.send_many(&["a", "", "c"]).is_err());
    assert_eq!(w.delivered, ["a"]);
    let mut c = Console::default();
    c.send_many(&["x", "y"]).unwrap();
    assert_eq!(c.sent.len(), 2);
}

#[test]
fn broadcast_over_trait_objects() {
    let mut ns: Vec<Box<dyn Notifier>> = vec![
        Box::new(Console::default()),
        Box::new(Email {
            to: "nope".into(),
            outbox: vec![],
        }),
        Box::new(Webhook {
            url: "https://ok".into(),
            delivered: vec![],
        }),
    ];
    let results = broadcast(&mut ns, "deploy done");
    let names: Vec<&str> = results.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(names, ["console", "email", "webhook"]);
    assert!(results[0].1.is_ok());
    assert!(results[1].1.is_err());
    assert!(results[2].1.is_ok());
}

#[test]
fn generic_send_all() {
    let mut emails = vec![
        Email {
            to: "a@x".into(),
            outbox: vec![],
        },
        Email {
            to: "bad".into(),
            outbox: vec![],
        },
        Email {
            to: "c@x".into(),
            outbox: vec![],
        },
    ];
    assert_eq!(send_all(&mut emails, "hi"), 2);
}
