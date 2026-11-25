use lab_2_4_drop_tracer::*;

#[test]
fn drop_is_implemented() {
    {
        let _x = Noisy("x");
    }
    assert_eq!(
        take_log(),
        vec!["x"],
        "Noisy's Drop impl should call log_drop"
    );
}

#[test]
fn prediction_matches_reality() {
    take_log();
    scenario();
    let actual = take_log();
    assert_eq!(actual.len(), 8, "every Noisy should be logged exactly once");
    assert_eq!(
        predicted_order(),
        actual,
        "your prediction vs the real drop order"
    );
}

#[test]
fn moved_values_drop_once() {
    let n = Noisy("moved");
    let m = n; // move: no drop here
    assert!(take_log().is_empty());
    drop(m);
    assert_eq!(take_log(), vec!["moved"]);
}
