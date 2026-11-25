//! Lab 7.2 — Zero-copy HTTP/1.1 request parser.
//!
//! ```text
//! POST /api/items HTTP/1.1\r\n
//! Host: example.com\r\n
//! Content-Type: application/json\r\n
//! \r\n
//! {"name":"x"}
//! ```
//! Every `&'a str` in `Request<'a>` points into the raw input — the only
//! allocation is the headers `Vec`. `OwnedRequest` is the same data, owned.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub version: &'a str,
    pub headers: Vec<(&'a str, &'a str)>,
    pub body: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// No "\r\n\r\n" separating head from body.
    Incomplete,
    /// Request line isn't `METHOD PATH VERSION`.
    BadRequestLine,
    /// A header line without ':'. Holds the offending line.
    BadHeader(String),
}

/// Parse a raw request. Header names and values are trimmed.
pub fn parse_request(raw: &str) -> Result<Request<'_>, ParseError> {
    // SOLUTION-START
    let (head, body) = raw.split_once("\r\n\r\n").ok_or(ParseError::Incomplete)?;
    let mut lines = head.split("\r\n");
    let request_line = lines.next().ok_or(ParseError::BadRequestLine)?;
    let mut parts = request_line.split(' ');
    let (Some(method), Some(path), Some(version), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return Err(ParseError::BadRequestLine);
    };
    if method.is_empty() || path.is_empty() || !version.starts_with("HTTP/") {
        return Err(ParseError::BadRequestLine);
    }
    let headers = lines
        .map(|line| {
            line.split_once(':')
                .map(|(k, v)| (k.trim(), v.trim()))
                .ok_or_else(|| ParseError::BadHeader(line.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Request {
        method,
        path,
        version,
        headers,
        body,
    })
    // SOLUTION-END
}

impl<'a> Request<'a> {
    /// Case-insensitive header lookup. Note the return type: the value lives
    /// as long as the RAW INPUT (`'a`), not as long as `&self`.
    pub fn header(&self, name: &str) -> Option<&'a str> {
        // SOLUTION-START
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|&(_, v)| v)
        // SOLUTION-END
    }
}

/// The owned twin: can be stored, sent to another thread, returned anywhere.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl From<Request<'_>> for OwnedRequest {
    fn from(r: Request<'_>) -> Self {
        // SOLUTION-START
        OwnedRequest {
            method: r.method.to_string(),
            path: r.path.to_string(),
            version: r.version.to_string(),
            headers: r
                .headers
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            body: r.body.to_string(),
        }
        // SOLUTION-END
    }
}
