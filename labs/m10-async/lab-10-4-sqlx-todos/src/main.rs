use lab_10_4_sqlx_todos::{app, connect};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://todos.db".to_string());
    let pool = connect(&url).await?;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!("listening on http://{} (db: {url})", listener.local_addr()?);
    axum::serve(listener, app(pool)).await?;
    Ok(())
}
