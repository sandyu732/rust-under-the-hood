use lab_1_2_overflow::*;

#[test]
fn checked_add() {
    assert_eq!(checked(250, 5), Some(255));
    assert_eq!(checked(250, 10), None);
}

#[test]
fn wrapping_add() {
    assert_eq!(wrapping(255, 1), 0);
    assert_eq!(wrapping(250, 10), 4);
}

#[test]
fn saturating_add() {
    assert_eq!(saturating(250, 10), 255);
    assert_eq!(saturating(1, 2), 3);
}

#[test]
fn overflowing_add() {
    assert_eq!(overflowing(255, 1), (0, true));
    assert_eq!(overflowing(1, 1), (2, false));
}

#[test]
fn conversions() {
    assert_eq!(to_u8_lossy(300), 44);
    assert_eq!(to_u8_lossy(-1), 255);
    assert_eq!(to_u8_checked(200), Ok(200));
    assert!(to_u8_checked(300).is_err());
    assert!(to_u8_checked(-1).is_err());
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "overflow")]
fn plain_add_panics_in_debug() {
    let a = std::hint::black_box(255u8);
    plain_add(a, 1);
}
