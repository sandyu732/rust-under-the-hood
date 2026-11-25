use lab_10_1_url_checker::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::time::{Instant, sleep};

fn urls(n: usize) -> Vec<String> {
    (0..n).map(|i| format!("https://site{i}.test")).collect()
}

/// Fake fetcher: "slow" URLs take 10s, "bad" fail, everything else takes 100ms.
/// Tracks the maximum number of concurrent requests.
fn fake(
    in_flight: Arc<AtomicUsize>,
    max_seen: Arc<AtomicUsize>,
) -> impl Fn(String) -> std::pin::Pin<Box<dyn Future<Output = Result<u16, String>> + Send>>
+ Clone
+ Send
+ 'static {
    move |url: String| {
        let in_flight = in_flight.clone();
        let max_seen = max_seen.clone();
        Box::pin(async move {
            let now = in_flight.fetch_add(1, Ordering::SeqCst) + 1;
            max_seen.fetch_max(now, Ordering::SeqCst);
            let result = if url.contains("slow") {
                sleep(Duration::from_secs(10)).await;
                Ok(200)
            } else if url.contains("bad") {
                Err("connection refused".to_string())
            } else {
                sleep(Duration::from_millis(100)).await;
                Ok(200)
            };
            in_flight.fetch_sub(1, Ordering::SeqCst);
            result
        })
    }
}

#[tokio::test(start_paused = true)]
async fn runs_concurrently_but_bounded() {
    let (inf, max) = (Arc::new(AtomicUsize::new(0)), Arc::new(AtomicUsize::new(0)));
    let start = Instant::now();
    let results = check_all(urls(20), 5, Duration::from_secs(1), fake(inf, max.clone())).await;
    let elapsed = start.elapsed();
    assert_eq!(results.len(), 20);
    assert!(
        max.load(Ordering::SeqCst) <= 5,
        "more than 5 requests in flight"
    );
    assert_eq!(
        max.load(Ordering::SeqCst),
        5,
        "concurrency limit should actually be used"
    );
    // 20 requests / 5 at a time × 100ms ≈ 400ms, not 2s sequentially.
    assert!(
        elapsed >= Duration::from_millis(400) && elapsed < Duration::from_millis(600),
        "took {elapsed:?}"
    );
}

#[tokio::test(start_paused = true)]
async fn timeouts_failures_and_order() {
    let (inf, max) = (Arc::new(AtomicUsize::new(0)), Arc::new(AtomicUsize::new(0)));
    let input = vec![
        "https://a.test".to_string(),
        "https://slow.test".into(),
        "https://bad.test".into(),
    ];
    let results = check_all(input.clone(), 10, Duration::from_secs(1), fake(inf, max)).await;
    let order: Vec<&str> = results.iter().map(|r| r.url.as_str()).collect();
    assert_eq!(order, input.iter().map(String::as_str).collect::<Vec<_>>());
    assert!(
        matches!(results[0].outcome, Outcome::Ok { status: 200, latency } if latency == Duration::from_millis(100))
    );
    assert_eq!(results[1].outcome, Outcome::Timeout);
    assert_eq!(
        results[2].outcome,
        Outcome::Failed("connection refused".into())
    );
    assert_eq!(summarize(&results), (1, 1, 1));
}

#[tokio::test]
async fn empty_input() {
    let (inf, max) = (Arc::new(AtomicUsize::new(0)), Arc::new(AtomicUsize::new(0)));
    assert!(
        check_all(vec![], 3, Duration::from_secs(1), fake(inf, max))
            .await
            .is_empty()
    );
}
