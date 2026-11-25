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
    // SOLUTION-START
    let permits = Arc::new(Semaphore::new(concurrency.max(1)));
    let handles: Vec<_> = urls
        .into_iter()
        .map(|url| {
            let permits = Arc::clone(&permits);
            let fetch = fetch.clone();
            tokio::spawn(async move {
                let _permit = permits.acquire_owned().await.expect("semaphore closed");
                let start = Instant::now();
                let outcome = match timeout(per_request, fetch(url.clone())).await {
                    Ok(Ok(status)) => Outcome::Ok {
                        status,
                        latency: start.elapsed(),
                    },
                    Ok(Err(e)) => Outcome::Failed(e),
                    Err(_) => Outcome::Timeout,
                };
                CheckResult { url, outcome }
            })
        })
        .collect();
    let mut results = Vec::with_capacity(handles.len());
    for h in handles {
        results.push(h.await.expect("task panicked"));
    }
    results
    // SOLUTION-END
}

/// Count results by kind: (ok, timeouts, failures).
pub fn summarize(results: &[CheckResult]) -> (usize, usize, usize) {
    // SOLUTION-START
    results
        .iter()
        .fold((0, 0, 0), |(o, t, f), r| match r.outcome {
            Outcome::Ok { .. } => (o + 1, t, f),
            Outcome::Timeout => (o, t + 1, f),
            Outcome::Failed(_) => (o, t, f + 1),
        })
    // SOLUTION-END
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
