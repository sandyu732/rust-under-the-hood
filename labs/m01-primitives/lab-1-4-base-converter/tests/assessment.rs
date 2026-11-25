use lab_1_4_base_converter::*;

fn args(a: &[&str]) -> Vec<String> {
    a.iter().map(|s| s.to_string()).collect()
}

#[test]
fn parses_bases() {
    assert_eq!(parse_base("hex"), Some(Base::Hex));
    assert_eq!(parse_base("BIN"), Some(Base::Bin));
    assert_eq!(parse_base("Oct"), Some(Base::Oct));
    assert_eq!(parse_base("dec"), None);
}

#[test]
fn formats() {
    assert_eq!(format_in_base(255, Base::Hex), "0xff");
    assert_eq!(format_in_base(5, Base::Bin), "0b101");
    assert_eq!(format_in_base(8, Base::Oct), "0o10");
    assert_eq!(format_in_base(0, Base::Hex), "0x0");
}

#[test]
fn run_happy_path() {
    assert_eq!(run(&args(&["255", "hex"])), Ok("0xff".to_string()));
}

#[test]
fn run_errors() {
    assert!(run(&args(&[])).is_err());
    assert!(run(&args(&["abc", "hex"])).unwrap_err().contains("abc"));
    assert!(run(&args(&["-1", "hex"])).is_err());
    assert!(
        run(&args(&["10", "base64"]))
            .unwrap_err()
            .contains("base64")
    );
}
