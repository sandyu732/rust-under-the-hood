use lab_9_3_pipeline::*;
use std::sync::mpsc;
use std::time::Duration;

#[test]
fn parses_records() {
    assert_eq!(
        parse_record("alice, 42"),
        Some(Record {
            name: "alice".into(),
            score: 42
        })
    );
    assert_eq!(parse_record("bob"), None);
    assert_eq!(parse_record(",3"), None);
    assert_eq!(parse_record("c,-1"), None);
}

#[test]
fn preserves_order_and_counts_rejects() {
    let input: Vec<String> = ["a,1", "junk", "b,2", "c,x", "d,4"]
        .map(String::from)
        .to_vec();
    let report = run_pipeline(input, 1);
    let names: Vec<&str> = report.records.iter().map(|r| r.name.as_str()).collect();
    assert_eq!(names, ["a", "b", "d"]);
    assert_eq!(report.rejected, 2);
}

#[test]
fn large_input_small_buffer_finishes() {
    // If shutdown is wrong (a Sender never dropped), this hangs: we run it on
    // a thread and give up after 10 seconds.
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let input = (0..100_000).map(|i| format!("user{i},{}", i % 100));
        tx.send(run_pipeline(input, 8)).unwrap();
    });
    let report = rx
        .recv_timeout(Duration::from_secs(10))
        .expect("pipeline did not shut down");
    assert_eq!(report.records.len(), 100_000);
    assert_eq!(report.records[99_999].name, "user99999");
}

#[test]
fn empty_input() {
    let report = run_pipeline(Vec::<String>::new(), 4);
    assert!(report.records.is_empty());
    assert_eq!(report.rejected, 0);
}
