//! Lab 10.2 — Async TCP chat server.
//!
//!   cargo run -p lab-10-2-chat-server        # listens on 127.0.0.1:8080
//!   nc localhost 8080                        # in two or more terminals
//!
//! Protocol (one message per line):
//! 1. The client's first line is its name. The server replies `* welcome <name>`.
//! 2. Every later line is sent to every OTHER client as `<name>: <line>`.
//! 3. When a client disconnects, the others get `* <name> left`.
//!
//! One task per connection; a `tokio::sync::broadcast` channel fans messages
//! out; `tokio::select!` waits on "my socket has a line" and "the channel has
//! a message" at the same time.

use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;

/// (sender id, text). The id lets a client skip its own messages.
type Msg = (usize, String);

/// Accept connections forever, spawning a task for each.
pub async fn run(listener: TcpListener) -> std::io::Result<()> {
    // SOLUTION-START
    let (tx, _) = broadcast::channel::<Msg>(256);
    let next_id = AtomicUsize::new(0);
    loop {
        let (stream, _addr) = listener.accept().await?;
        let id = next_id.fetch_add(1, Ordering::Relaxed);
        let tx = tx.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, id, tx).await {
                eprintln!("client {id}: {e}");
            }
        });
    }
    // SOLUTION-END
}

/// Serve one client until it disconnects.
pub async fn handle_client(
    stream: TcpStream,
    id: usize,
    tx: broadcast::Sender<Msg>,
) -> std::io::Result<()> {
    // SOLUTION-START
    let (read_half, mut write_half) = stream.into_split();
    let mut lines = BufReader::new(read_half).lines();

    let Some(name) = lines.next_line().await? else {
        return Ok(());
    };
    let name = name.trim().to_string();
    let mut rx = tx.subscribe(); // subscribe BEFORE welcoming, so no message is missed
    write_half
        .write_all(format!("* welcome {name}\n").as_bytes())
        .await?;

    loop {
        tokio::select! {
            line = lines.next_line() => match line? {
                Some(text) => { let _ = tx.send((id, format!("{name}: {text}"))); }
                None => break, // client disconnected
            },
            msg = rx.recv() => match msg {
                Ok((from, text)) if from != id => {
                    write_half.write_all(format!("{text}\n").as_bytes()).await?;
                }
                Ok(_) => {} // our own message
                Err(broadcast::error::RecvError::Lagged(_)) => {} // too slow: skip missed messages
                Err(broadcast::error::RecvError::Closed) => break,
            },
        }
    }
    let _ = tx.send((id, format!("* {name} left")));
    Ok(())
    // SOLUTION-END
}
