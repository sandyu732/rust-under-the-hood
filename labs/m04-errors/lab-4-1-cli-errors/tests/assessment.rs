use lab_4_1_cli_errors::*;

fn args(a: &[&str]) -> Vec<String> {
    a.iter().map(|s| s.to_string()).collect()
}

#[test]
fn converts() {
    assert_eq!(run(&args(&["255", "hex"])).unwrap(), "0xff");
    assert_eq!(run(&args(&["5", "bin"])).unwrap(), "0b101");
}

#[test]
fn error_messages_are_helpful() {
    assert!(run(&args(&[])).unwrap_err().to_string().contains("number"));
    assert!(
        run(&args(&["10"]))
            .unwrap_err()
            .to_string()
            .contains("bin|oct|hex")
    );
    assert!(
        run(&args(&["ten", "hex"]))
            .unwrap_err()
            .to_string()
            .contains("'ten'")
    );
    assert!(
        run(&args(&["10", "b64"]))
            .unwrap_err()
            .to_string()
            .contains("b64")
    );
    assert!(run(&args(&["1", "hex", "extra"])).is_err());
}

#[test]
fn no_unwrap_or_error_match_in_source() {
    let src = include_str!("../src/lib.rs");
    let code: String = src
        .lines()
        .filter(|l| !l.trim_start().starts_with("//"))
        .collect();
    assert!(
        !code.contains(".unwrap()"),
        "remove .unwrap() from src/lib.rs"
    );
    assert!(
        !code.contains("Err(e) =>"),
        "use ? instead of matching on errors"
    );
}
