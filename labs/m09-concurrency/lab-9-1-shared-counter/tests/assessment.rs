use lab_9_1_shared_counter::*;

#[test]
fn arc_mutex() {
    assert_eq!(count_with_arc_mutex(8, 1_000), 8_000);
}

#[test]
fn scoped() {
    assert_eq!(count_with_scope(8, 1_000), 8_000);
}

#[test]
fn atomic() {
    assert_eq!(count_with_atomic(8, 10_000), 80_000);
}

#[test]
fn deterministic_over_many_runs() {
    for _ in 0..20 {
        assert_eq!(count_with_atomic(4, 500), 2_000);
        assert_eq!(count_with_scope(4, 500), 2_000);
    }
}

#[test]
fn parallel_sum_matches_sequential() {
    let data: Vec<u64> = (1..=100_000).collect();
    let expected: u64 = data.iter().sum();
    for chunks in [1, 3, 8, 1000] {
        assert_eq!(parallel_sum(&data, chunks), expected, "chunks = {chunks}");
    }
    assert_eq!(parallel_sum(&[], 4), 0);
}
