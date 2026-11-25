use lab_5_2_vec2::Vec2;

#[test]
fn the_goal_expression() {
    let v: Vec2 = (1.0, 2.0).into();
    let w = v + v * 3.0 - Vec2::default();
    assert_eq!(w, Vec2::new(4.0, 8.0));
}

#[test]
fn operators() {
    let a = Vec2::new(1.0, -2.0);
    assert_eq!(-a, Vec2::new(-1.0, 2.0));
    assert_eq!(2.0 * a, a * 2.0);
    let mut acc = Vec2::default();
    acc += a;
    acc += a;
    assert_eq!(acc, Vec2::new(2.0, -4.0));
}

#[test]
fn geometry() {
    assert_eq!(Vec2::new(3.0, 4.0).length(), 5.0);
    assert_eq!(Vec2::new(1.0, 2.0).dot(Vec2::new(3.0, 4.0)), 11.0);
    let n = Vec2::new(0.0, 5.0).normalized().unwrap();
    assert_eq!(n, Vec2::new(0.0, 1.0));
    assert_eq!(Vec2::default().normalized(), None);
}

#[test]
fn display() {
    assert_eq!(Vec2::new(1.0, 2.5).to_string(), "(1, 2.5)");
}

#[test]
fn is_copy() {
    let a = Vec2::new(1.0, 1.0);
    let b = a; // Copy, not move
    assert_eq!(a + b, Vec2::new(2.0, 2.0));
}
