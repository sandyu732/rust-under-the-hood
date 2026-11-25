use lab_4_4_csv_stats::*;

const GOOD: &str = "name,price,qty\napple,1.5,3\npear,2.5,1\n\nfig,4,2\n";
const MIXED: &str = "name,price\na,1\nb,oops\nc\nd,3\n";

#[test]
fn strict_parses_named_column() {
    assert_eq!(parse_strict(GOOD, "price"), Ok(vec![1.5, 2.5, 4.0]));
    assert_eq!(parse_strict(GOOD, "qty"), Ok(vec![3.0, 1.0, 2.0]));
}

#[test]
fn strict_fails_fast_with_line_number() {
    assert_eq!(
        parse_strict(MIXED, "price"),
        Err(CsvError::BadValue {
            line: 3,
            value: "oops".into()
        })
    );
    assert_eq!(
        parse_strict(GOOD, "colour"),
        Err(CsvError::MissingColumn("colour".into()))
    );
    assert_eq!(parse_strict("", "x"), Err(CsvError::Empty));
}

#[test]
fn lenient_collects_errors() {
    let (values, errors) = parse_lenient(MIXED, "price").unwrap();
    assert_eq!(values, vec![1.0, 3.0]);
    assert_eq!(
        errors,
        vec![
            CsvError::BadValue {
                line: 3,
                value: "oops".into()
            },
            CsvError::ShortRow { line: 4 }
        ]
    );
    assert!(parse_lenient(MIXED, "nope").is_err());
}

#[test]
fn computes_stats() {
    let s = stats(&[3.0, 1.0, 4.0, 2.0]).unwrap();
    assert_eq!(
        s,
        Stats {
            count: 4,
            min: 1.0,
            max: 4.0,
            mean: 2.5,
            median: 2.5
        }
    );
    assert_eq!(stats(&[5.0, 1.0, 3.0]).unwrap().median, 3.0);
    assert_eq!(stats(&[]), None);
}

#[test]
fn error_messages() {
    assert_eq!(
        CsvError::ShortRow { line: 4 }.to_string(),
        "line 4: row has too few fields"
    );
}
