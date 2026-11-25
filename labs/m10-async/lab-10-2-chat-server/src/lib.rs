#![allow(unused)] // TODO: delete this line once the lab is done
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
    todo!("lab-10-2-chat-server: implement me")
}

/// Serve one client until it disconnects.
pub async fn handle_client(
    stream: TcpStream,
    id: usize,
    tx: broadcast::Sender<Msg>,
) -> std::io::Result<()> {
    todo!("lab-10-2-chat-server: implement me")
}
