#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 6.3 — Log analyzer.
//!
//! Line format: `2026-11-01T10:00:00Z INFO /api/users 123ms`
//!
//!   cargo run --release -p lab-6-3-log-analyzer -- gen 100000 > /tmp/app.log
//!   cargo run --release -p lab-6-3-log-analyzer -- analyze /tmp/app.log

use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogLine {
    pub timestamp: String,
    pub level: Level,
    pub endpoint: String,
    pub latency_ms: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    #[error("expected 4 fields, found {0}")]
    FieldCount(usize),
    #[error("unknown level `{0}`")]
    Level(String),
    #[error("bad latency `{0}`")]
    Latency(String),
}

/// "DEBUG" | "INFO" | "WARN" | "ERROR" (exact, uppercase).
impl FromStr for Level {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("lab-6-3-log-analyzer: implement me")
    }
}

/// Parse one line. Latency must end in "ms".
pub fn parse_line(line: &str) -> Result<LogLine, ParseError> {
    todo!("lab-6-3-log-analyzer: implement me")
}

pub fn count_by_level(lines: &[LogLine]) -> HashMap<Level, usize> {
    todo!("lab-6-3-log-analyzer: implement me")
}

/// The `n` endpoints with the highest AVERAGE latency, highest first.
pub fn slowest_endpoints(lines: &[LogLine], n: usize) -> Vec<(String, f64)> {
    todo!("lab-6-3-log-analyzer: implement me")
}

/// p95 latency per endpoint using the nearest-rank method:
/// sort the latencies, take the element at index `ceil(0.95 * len) - 1`.
pub fn p95_by_endpoint(lines: &[LogLine]) -> BTreeMap<String, u32> {
    todo!("lab-6-3-log-analyzer: implement me")
}

/// Given: deterministic fake log generator (no `rand` crate needed).
pub fn generate(n: usize) -> String {
    const ENDPOINTS: [&str; 5] = [
        "/api/users",
        "/api/orders",
        "/api/search",
        "/health",
        "/api/login",
    ];
    const LEVELS: [&str; 4] = ["DEBUG", "INFO", "WARN", "ERROR"];
    let mut seed: u64 = 42;
    let mut next = move || {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (seed >> 33) as usize
    };
    let mut out = String::with_capacity(n * 48);
    for i in 0..n {
        let ep = ENDPOINTS[next() % ENDPOINTS.len()];
        let lvl = LEVELS[(next() % 10).min(3)];
        let lat = 5 + next() % (if ep == "/api/search" { 900 } else { 200 });
        out.push_str(&format!(
            "2026-11-01T10:{:02}:{:02}Z {lvl} {ep} {lat}ms\n",
            (i / 60) % 60,
            i % 60
        ));
    }
    out
}
