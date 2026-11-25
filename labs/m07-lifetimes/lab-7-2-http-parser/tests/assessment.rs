use lab_7_2_http_parser::*;

const RAW: &str = "POST /api/items HTTP/1.1\r\nHost: example.com\r\nContent-Type:  application/json \r\n\r\n{\"name\":\"x\"}";

fn points_into(whole: &str, part: &str) -> bool {
    let w = whole.as_bytes().as_ptr_range();
    let p = part.as_bytes().as_ptr_range();
    part.is_empty() || (w.start <= p.start && p.end <= w.end)
}

#[test]
fn parses_request() {
    let r = parse_request(RAW).unwrap();
    assert_eq!(r.method, "POST");
    assert_eq!(r.path, "/api/items");
    assert_eq!(r.version, "HTTP/1.1");
    assert_eq!(
        r.headers,
        vec![
            ("Host", "example.com"),
            ("Content-Type", "application/json")
        ]
    );
    assert_eq!(r.body, "{\"name\":\"x\"}");
}

#[test]
fn is_zero_copy() {
    let owned_input = RAW.to_string();
    let r = parse_request(&owned_input).unwrap();
    for s in [
        r.method,
        r.path,
        r.version,
        r.body,
        r.headers[0].0,
        r.headers[1].1,
    ] {
        assert!(
            points_into(&owned_input, s),
            "`{s}` was copied instead of borrowed"
        );
    }
}

#[test]
fn header_lookup_outlives_the_request_struct() {
    let host;
    {
        let r = parse_request(RAW).unwrap();
        host = r.header("HOST");
    }
    assert_eq!(host, Some("example.com"));
    assert_eq!(parse_request(RAW).unwrap().header("missing"), None);
}

#[test]
fn errors() {
    assert_eq!(
        parse_request("GET / HTTP/1.1\r\n"),
        Err(ParseError::Incomplete)
    );
    assert_eq!(
        parse_request("GET /\r\n\r\n"),
        Err(ParseError::BadRequestLine)
    );
    assert_eq!(
        parse_request("GET / FTP\r\n\r\n"),
        Err(ParseError::BadRequestLine)
    );
    assert_eq!(
        parse_request("GET / HTTP/1.1\r\nnocolon\r\n\r\n"),
        Err(ParseError::BadHeader("nocolon".into()))
    );
}

#[test]
fn converts_to_owned() {
    let owned: OwnedRequest = {
        let raw = RAW.to_string();
        parse_request(&raw).unwrap().into()
    }; // raw dropped — the owned copy is independent
    assert_eq!(owned.path, "/api/items");
    assert_eq!(
        owned.headers[0],
        ("Host".to_string(), "example.com".to_string())
    );
}
