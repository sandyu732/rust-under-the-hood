use lab_10_1_url_checker::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: url-checker <file-with-one-url-per-line>")?;
    let urls: Vec<String> = tokio::fs::read_to_string(path)
        .await?
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect();
    let client = reqwest::Client::new();
    let results = check_all(urls, 10, Duration::from_secs(5), move |u| {
        http_status(client.clone(), u)
    })
    .await;
    for r in &results {
        println!("{:<50} {:?}", r.url, r.outcome);
    }
    let (ok, timeouts, failed) = summarize(&results);
    println!("\nok: {ok}, timeouts: {timeouts}, failed: {failed}");
    Ok(())
}
