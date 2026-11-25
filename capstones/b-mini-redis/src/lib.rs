#![allow(unused)] // TODO: delete this line once the lab is done
//! Capstone B — mini-Redis: an async key-value server with TTLs and an
//! append-only file (AOF) for persistence.
//!
//!   cargo run -p capstone-b-mini-redis -- --aof data.aof
//!   nc localhost 6380
//!   SET greeting hello EX 60
//!   GET greeting
//!
//! Text protocol, one command per line (case-insensitive command names):
//!
//! | Command                | Reply                          |
//! |------------------------|--------------------------------|
//! | `PING`                 | `PONG`                         |
//! | `SET key value`        | `OK`                           |
//! | `SET key value EX n`   | `OK` (expires after n seconds) |
//! | `GET key`              | the value, or `(nil)`          |
//! | `DEL key [key ...]`    | `(integer) <number deleted>`   |
//! | `INCR key`             | `(integer) <new value>`        |
//! | anything invalid       | `ERR <message>`                |
//!
//! Design: `Store` is plain synchronous logic (easy to unit-test, takes `now`
//! as a parameter so TTL tests don't sleep). `Server` wraps it in a `Mutex`
//! plus an optional AOF file; `serve` runs one tokio task per connection.
//! Mutating commands are appended to the AOF; `Server::open` replays it.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Ping,
    Get(String),
    Set {
        key: String,
        value: String,
        ttl: Option<Duration>,
    },
    Del(Vec<String>),
    Incr(String),
}

impl Command {
    /// Does this command change data (and so belong in the AOF)?
    pub fn is_write(&self) -> bool {
        todo!("b-mini-redis: implement me")
    }
}

/// Parse one line. Errors are the message for `ERR <message>`.
pub fn parse_command(line: &str) -> Result<Command, String> {
    todo!("b-mini-redis: implement me")
}

#[derive(Debug, Default)]
pub struct Store {
    data: HashMap<String, (String, Option<Instant>)>,
}

impl Store {
    pub fn new() -> Self {
        Self::default()
    }

    /// Execute `cmd` as of time `now` and return the reply line (no newline).
    /// Expired keys behave as if they don't exist (lazy expiry).
    pub fn apply(&mut self, cmd: &Command, now: Instant) -> String {
        todo!("b-mini-redis: implement me")
    }
}

pub struct Server {
    store: Mutex<Store>,
    aof: Option<Mutex<File>>,
}

impl Server {
    /// Create a server. With an AOF path: replay every line of the file into
    /// a fresh store (creating the file if missing), then keep it open for appending.
    pub fn open(aof: Option<&Path>) -> io::Result<Arc<Server>> {
        todo!("b-mini-redis: implement me")
    }

    /// Parse and execute one line; append successful writes to the AOF.
    pub fn handle_line(&self, line: &str) -> String {
        todo!("b-mini-redis: implement me")
    }
}

/// Accept connections forever, one task per client.
pub async fn serve(listener: TcpListener, server: Arc<Server>) -> io::Result<()> {
    todo!("b-mini-redis: implement me")
}

/// Given: parse `--aof <path>` and `--addr <host:port>` from argv.
pub fn parse_args(args: &[String]) -> (String, Option<PathBuf>) {
    let mut addr = "127.0.0.1:6380".to_string();
    let mut aof = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--addr" => addr = it.next().cloned().unwrap_or(addr),
            "--aof" => aof = it.next().map(PathBuf::from),
            _ => {}
        }
    }
    (addr, aof)
}
