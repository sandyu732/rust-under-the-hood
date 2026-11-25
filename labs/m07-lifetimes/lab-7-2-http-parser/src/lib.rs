#![allow(unused)] // TODO: delete this line once the lab is done
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
    todo!("lab-7-2-http-parser: implement me")
}

impl<'a> Request<'a> {
    /// Case-insensitive header lookup. Note the return type: the value lives
    /// as long as the RAW INPUT (`'a`), not as long as `&self`.
    pub fn header(&self, name: &str) -> Option<&'a str> {
        todo!("lab-7-2-http-parser: implement me")
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
        todo!("lab-7-2-http-parser: implement me")
    }
}
