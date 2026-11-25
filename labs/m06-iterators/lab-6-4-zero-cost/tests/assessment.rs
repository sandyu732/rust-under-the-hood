use lab_6_4_zero_cost::*;

#[test]
fn all_three_agree() {
    let data: Vec<u64> = (0..10_000).map(|x| x % 1000).collect();
    let expected = sum_odd_squares_index(&data);
    assert_eq!(sum_odd_squares_for(&data), expected);
    assert_eq!(sum_odd_squares_iter(&data), expected);
}

#[test]
fn known_values() {
    assert_eq!(sum_odd_squares_iter(&[1, 2, 3, 4, 5]), 35);
    assert_eq!(sum_odd_squares_index(&[]), 0);
    assert_eq!(sum_odd_squares_for(&[2, 4]), 0);
}
