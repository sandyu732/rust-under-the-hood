use lab_6_5_closures::*;

#[test]
fn fn_closures() {
    let offset = 10;
    assert_eq!(apply(|x| x + offset, 5), 15);
    assert_eq!(apply_twice(|x| x * 3, 2), 18);
}

#[test]
fn fn_mut_closures() {
    let mut log = Vec::new();
    call_n_times(|| log.push(log.len()), 3);
    assert_eq!(log, [0, 1, 2]);
}

#[test]
fn fn_once_closures() {
    let name = String::from("ferris");
    // This closure moves `name` out, so it is only FnOnce.
    let greeting = run_once(move || name + " says hi");
    assert_eq!(greeting, "ferris says hi");
    // run_once also accepts plain Fn closures:
    assert_eq!(run_once(|| "x".to_string()), "x");
}

#[test]
fn returned_closures() {
    let mut next = make_counter();
    assert_eq!((next(), next(), next()), (1, 2, 3));
    let mut other = make_counter();
    assert_eq!(other(), 1, "each counter has its own state");

    let add5 = make_adder(5);
    assert_eq!(add5(1), 6);

    let shout_len = compose(|s: &str| s.to_uppercase(), |s: String| s.len());
    assert_eq!(shout_len("hey"), 3);
    let inc_then_double = compose(make_adder(1), |x| x * 2);
    assert_eq!(inc_then_double(4), 10);
}
