use lab_4_2_config_loader::*;
use std::error::Error;
use std::io::Write;

#[test]
fn parses_full_config_with_comments() {
    let text = "# server\nhost = example.com\n\nport=8080   # http\nworkers = 8\n";
    assert_eq!(
        parse_config(text).unwrap(),
        Config {
            host: "example.com".into(),
            port: 8080,
            workers: 8
        }
    );
}

#[test]
fn workers_defaults_to_four() {
    assert_eq!(parse_config("host=a\nport=1").unwrap().workers, 4);
}

#[test]
fn each_error_variant() {
    assert!(matches!(
        parse_config("host=a\nport"),
        Err(ConfigError::BadLine { line: 2 })
    ));
    assert!(matches!(
        parse_config("port=1"),
        Err(ConfigError::Missing("host"))
    ));
    assert!(matches!(
        parse_config("host=a"),
        Err(ConfigError::Missing("port"))
    ));
    assert!(matches!(
        parse_config("host=a\nport=99999"),
        Err(ConfigError::InvalidValue { ref key, ref value }) if key == "port" && value == "99999"
    ));
    assert!(matches!(
        parse_config("host=a\nport=1\ncolour=blue"),
        Err(ConfigError::UnknownKey { ref key, line: 3 }) if key == "colour"
    ));
}

#[test]
fn display_messages() {
    assert_eq!(
        ConfigError::BadLine { line: 2 }.to_string(),
        "line 2: expected `key = value`"
    );
    assert_eq!(
        ConfigError::Missing("host").to_string(),
        "missing required key `host`"
    );
    let e = parse_config("host=a\nport=abc").unwrap_err();
    assert_eq!(e.to_string(), "invalid value `abc` for `port`");
}

#[test]
fn io_errors_convert_and_keep_source() {
    let err = load_config("/definitely/not/here.conf").unwrap_err();
    assert!(matches!(err, ConfigError::Io(_)));
    assert!(
        err.source().is_some(),
        "Io variant should expose the io::Error as source()"
    );
    assert!(ConfigError::Missing("x").source().is_none());
}

#[test]
fn loads_from_file() {
    let mut f = tempfile::NamedTempFile::new().unwrap();
    writeln!(f, "host = localhost\nport = 3000").unwrap();
    assert_eq!(load_config(f.path()).unwrap().port, 3000);
}
