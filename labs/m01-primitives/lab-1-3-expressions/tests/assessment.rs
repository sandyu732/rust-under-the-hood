use lab_1_3_expressions::*;

#[test]
fn fizzbuzz_values() {
    assert_eq!(fizzbuzz(1), "1");
    assert_eq!(fizzbuzz(3), "Fizz");
    assert_eq!(fizzbuzz(10), "Buzz");
    assert_eq!(fizzbuzz(30), "FizzBuzz");
    assert_eq!(fizzbuzz_upto(5), ["1", "2", "Fizz", "4", "Buzz"]);
}

#[test]
fn temperatures() {
    assert!((celsius_to_fahrenheit(100.0) - 212.0).abs() < 1e-9);
    assert!((fahrenheit_to_celsius(-40.0) + 40.0).abs() < 1e-9);
    assert!((fahrenheit_to_celsius(celsius_to_fahrenheit(37.0)) - 37.0).abs() < 1e-9);
}

#[test]
fn classify_numbers() {
    assert_eq!(classify(-7), "negative");
    assert_eq!(classify(0), "zero");
    assert_eq!(classify(i32::MAX), "positive");
}

#[test]
fn grades() {
    assert_eq!(grade(100), Some('A'));
    assert_eq!(grade(85), Some('B'));
    assert_eq!(grade(70), Some('C'));
    assert_eq!(grade(69), Some('D'));
    assert_eq!(grade(0), Some('F'));
    assert_eq!(grade(101), None);
}

#[test]
fn collatz() {
    assert_eq!(collatz_steps(1), 0);
    assert_eq!(collatz_steps(6), 8);
    assert_eq!(collatz_steps(27), 111);
}
