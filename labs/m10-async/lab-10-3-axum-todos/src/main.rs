use lab_10_3_axum_todos::{Db, app};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!("listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app(Db::default())).await
}
