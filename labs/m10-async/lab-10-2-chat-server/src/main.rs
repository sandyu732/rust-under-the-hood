#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("chat server listening on {addr}");
    lab_10_2_chat_server::run(listener).await
}
