use capstone_b_mini_redis::{Server, parse_args, serve};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (addr, aof) = parse_args(&args);
    let server = Server::open(aof.as_deref())?;
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("mini-redis on {addr} (aof: {aof:?})");
    serve(listener, server).await
}
