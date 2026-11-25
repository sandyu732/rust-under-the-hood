use lab_6_3_log_analyzer::*;

fn lines(text: &str) -> Vec<LogLine> {
    text.lines().map(|l| parse_line(l).unwrap()).collect()
}

const SAMPLE: &str = "\
t1 INFO /a 10ms
t2 INFO /a 30ms
t3 ERROR /b 100ms
t4 WARN /c 50ms
t5 INFO /c 70ms";

#[test]
fn parses_a_line() {
    assert_eq!(
        parse_line("2026-11-01T10:00:00Z INFO /api/users 123ms"),
        Ok(LogLine {
            timestamp: "2026-11-01T10:00:00Z".into(),
            level: Level::Info,
            endpoint: "/api/users".into(),
            latency_ms: 123,
        })
    );
}

#[test]
fn parse_errors() {
    assert_eq!(parse_line("t INFO /a"), Err(ParseError::FieldCount(3)));
    assert_eq!(
        parse_line("t TRACE /a 1ms"),
        Err(ParseError::Level("TRACE".into()))
    );
    assert_eq!(
        parse_line("t INFO /a 12s"),
        Err(ParseError::Latency("12s".into()))
    );
}

#[test]
fn counts_levels() {
    let c = count_by_level(&lines(SAMPLE));
    assert_eq!(c[&Level::Info], 3);
    assert_eq!(c[&Level::Error], 1);
    assert_eq!(c.get(&Level::Debug), None);
}

#[test]
fn slowest_by_average() {
    let s = slowest_endpoints(&lines(SAMPLE), 2);
    assert_eq!(s, vec![("/b".to_string(), 100.0), ("/c".to_string(), 60.0)]);
}

#[test]
fn p95_nearest_rank() {
    let text: String = (1..=20).map(|i| format!("t INFO /x {i}ms\n")).collect();
    let p = p95_by_endpoint(&lines(&text));
    assert_eq!(p["/x"], 19);
    assert_eq!(p95_by_endpoint(&lines(SAMPLE))["/a"], 30);
}

#[test]
fn handles_generated_logs() {
    let text = generate(10_000);
    let parsed: Result<Vec<_>, _> = text.lines().map(parse_line).collect();
    let parsed = parsed.expect("generated logs should all parse");
    assert_eq!(parsed.len(), 10_000);
    assert_eq!(p95_by_endpoint(&parsed).len(), 5);
}
