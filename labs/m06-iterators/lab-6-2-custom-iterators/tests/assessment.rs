use lab_6_2_custom_iterators::*;

#[test]
fn fib_sequence() {
    let first: Vec<u64> = fib().take(8).collect();
    assert_eq!(first, [1, 1, 2, 3, 5, 8, 13, 21]);
}

#[test]
fn fib_terminates_instead_of_overflowing() {
    let n = fib().count();
    assert!(
        n > 90 && n < 100,
        "fib should stop near u64::MAX, got {n} items"
    );
}

#[test]
fn project_euler_2() {
    assert_eq!(even_fib_sum_below(4_000_000), 4_613_732);
}

#[test]
fn ten_digits() {
    assert_eq!(first_fib_with_digits(10), Some(1_134_903_170));
    assert_eq!(first_fib_with_digits(1), Some(1));
}

#[test]
fn step_ranges() {
    assert_eq!(step_range(0, 10, 3).collect::<Vec<_>>(), [0, 3, 6, 9]);
    assert_eq!(step_range(10, 0, -3).collect::<Vec<_>>(), [10, 7, 4, 1]);
    assert_eq!(step_range(0, 5, 0).count(), 0);
    assert_eq!(step_range(5, 5, 1).count(), 0);
}

#[test]
fn windows() {
    assert_eq!(window_sums(&[1, 2, 3, 4], 2), [3, 5, 7]);
    assert!(window_sums(&[1, 2], 3).is_empty());
}
