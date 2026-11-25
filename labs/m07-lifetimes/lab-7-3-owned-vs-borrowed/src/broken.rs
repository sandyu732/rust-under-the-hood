use lab_7_2_http_parser::{Request, parse_request};

/// E0515: returns a value referencing data owned by the current function.
/// `raw` is dropped when this function returns, but the Request borrows it.
pub fn make_request() -> Request<'static> {
    let raw = crate::raw_request(1);
    parse_request(&raw).unwrap()
}
