#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 10.1 — Concurrent URL checker with bounded concurrency and timeouts.
//!
//!   cargo run -p lab-10-1-url-checker -- urls.txt
//!
//! `check_all` is generic over the fetch function so the assessment can use
//! a fake, instant, deterministic fetcher (with tokio's paused clock), while
//! `main` uses real HTTP via reqwest.
//!
//! Tools: `tokio::sync::Semaphore` (or `futures::stream::iter(..).buffered(n)`),
//! `tokio::time::timeout`, `tokio::time::Instant`.

use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::{Instant, timeout};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Ok { status: u16, latency: Duration },
    Timeout,
    Failed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckResult {
    pub url: String,
    pub outcome: Outcome,
}

/// Check every URL with at most `concurrency` requests in flight, each
/// limited to `per_request`. Results are returned in INPUT order.
pub async fn check_all<F, Fut>(
    urls: Vec<String>,
    concurrency: usize,
    per_request: Duration,
    fetch: F,
) -> Vec<CheckResult>
where
    F: Fn(String) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = Result<u16, String>> + Send + 'static,
{
    todo!("lab-10-1-url-checker: implement me")
}

/// Count results by kind: (ok, timeouts, failures).
pub fn summarize(results: &[CheckResult]) -> (usize, usize, usize) {
    todo!("lab-10-1-url-checker: implement me")
}

/// Given: the real fetcher used by `main`.
pub async fn http_status(client: reqwest::Client, url: String) -> Result<u16, String> {
    client
        .get(&url)
        .send()
        .await
        .map(|r| r.status().as_u16())
        .map_err(|e| e.to_string())
}
