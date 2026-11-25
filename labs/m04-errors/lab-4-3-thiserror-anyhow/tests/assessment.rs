use lab_4_3_thiserror_anyhow::*;
use std::error::Error;

#[test]
fn messages_come_from_error_attributes() {
    assert_eq!(
        parse_config("oops").unwrap_err().to_string(),
        "line 1: expected `key = value`"
    );
    assert_eq!(
        parse_config("host=a").unwrap_err().to_string(),
        "missing required key `port`"
    );
    assert_eq!(
        parse_config("host=a\nport=1\nx=1").unwrap_err().to_string(),
        "line 3: unknown key `x`"
    );
}

#[test]
fn from_io_error_works_with_question_mark() {
    let err = load_config("/no/such/file").unwrap_err();
    assert!(matches!(err, ConfigError::Io(_)));
    assert!(err.source().is_some());
}

#[test]
fn anyhow_adds_context_and_keeps_the_chain() {
    let err = load_for_app("/no/such/file").unwrap_err();
    assert_eq!(err.to_string(), "loading config from /no/such/file");
    assert!(
        err.chain().count() >= 2,
        "context should wrap the original error"
    );
    assert!(err.root_cause().downcast_ref::<std::io::Error>().is_some());
    assert!(
        err.downcast_ref::<ConfigError>().is_some(),
        "the typed error is still inside"
    );
}

#[test]
fn happy_path() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("app.conf");
    std::fs::write(&p, "host = h\nport = 1\nworkers = 2").unwrap();
    let cfg = load_for_app(p.to_str().unwrap()).unwrap();
    assert_eq!(
        cfg,
        Config {
            host: "h".into(),
            port: 1,
            workers: 2
        }
    );
}
