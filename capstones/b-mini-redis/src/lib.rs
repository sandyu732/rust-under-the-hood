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
        // SOLUTION-START
        matches!(
            self,
            Command::Set { .. } | Command::Del(_) | Command::Incr(_)
        )
        // SOLUTION-END
    }
}

/// Parse one line. Errors are the message for `ERR <message>`.
pub fn parse_command(line: &str) -> Result<Command, String> {
    // SOLUTION-START
    let parts: Vec<&str> = line.split_whitespace().collect();
    let Some((name, args)) = parts.split_first() else {
        return Err("empty command".into());
    };
    match (name.to_ascii_uppercase().as_str(), args) {
        ("PING", []) => Ok(Command::Ping),
        ("GET", [key]) => Ok(Command::Get(key.to_string())),
        ("SET", [key, value]) => Ok(Command::Set {
            key: key.to_string(),
            value: value.to_string(),
            ttl: None,
        }),
        ("SET", [key, value, ex, secs]) if ex.eq_ignore_ascii_case("EX") => {
            let secs: u64 = secs
                .parse()
                .map_err(|_| "EX needs a positive integer".to_string())?;
            if secs == 0 {
                return Err("EX needs a positive integer".into());
            }
            Ok(Command::Set {
                key: key.to_string(),
                value: value.to_string(),
                ttl: Some(Duration::from_secs(secs)),
            })
        }
        ("DEL", keys) if !keys.is_empty() => {
            Ok(Command::Del(keys.iter().map(|k| k.to_string()).collect()))
        }
        ("INCR", [key]) => Ok(Command::Incr(key.to_string())),
        ("PING" | "GET" | "SET" | "DEL" | "INCR", _) => {
            Err(format!("wrong number of arguments for {name}"))
        }
        _ => Err(format!("unknown command '{name}'")),
    }
    // SOLUTION-END
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
        // SOLUTION-START
        match cmd {
            Command::Ping => "PONG".into(),
            Command::Get(key) => self
                .live(key, now)
                .map_or("(nil)".into(), |v| v.to_string()),
            Command::Set { key, value, ttl } => {
                self.data
                    .insert(key.clone(), (value.clone(), ttl.map(|t| now + t)));
                "OK".into()
            }
            Command::Del(keys) => {
                let n = keys
                    .iter()
                    .filter(|k| self.live(k, now).is_some() && self.data.remove(*k).is_some())
                    .count();
                format!("(integer) {n}")
            }
            Command::Incr(key) => {
                let current = self.live(key, now).unwrap_or("0");
                let Ok(n) = current.parse::<i64>() else {
                    return "ERR value is not an integer".into();
                };
                let Some(next) = n.checked_add(1) else {
                    return "ERR increment would overflow".into();
                };
                let expiry = self
                    .data
                    .get(key)
                    .and_then(|(_, e)| *e)
                    .filter(|e| *e > now);
                self.data.insert(key.clone(), (next.to_string(), expiry));
                format!("(integer) {next}")
            }
        }
        // SOLUTION-END
    }

    // SOLUTION-ONLY-START
    fn live(&self, key: &str, now: Instant) -> Option<&str> {
        match self.data.get(key) {
            Some((v, None)) => Some(v),
            Some((v, Some(exp))) if *exp > now => Some(v),
            _ => None,
        }
    }
    // SOLUTION-ONLY-END
}

pub struct Server {
    store: Mutex<Store>,
    aof: Option<Mutex<File>>,
}

impl Server {
    /// Create a server. With an AOF path: replay every line of the file into
    /// a fresh store (creating the file if missing), then keep it open for appending.
    pub fn open(aof: Option<&Path>) -> io::Result<Arc<Server>> {
        // SOLUTION-START
        let mut store = Store::new();
        let file = match aof {
            Some(path) => {
                if path.exists() {
                    let now = Instant::now();
                    for line in io::BufReader::new(File::open(path)?).lines() {
                        if let Ok(cmd) = parse_command(&line?) {
                            store.apply(&cmd, now);
                        }
                    }
                }
                Some(Mutex::new(
                    OpenOptions::new().create(true).append(true).open(path)?,
                ))
            }
            None => None,
        };
        Ok(Arc::new(Server {
            store: Mutex::new(store),
            aof: file,
        }))
        // SOLUTION-END
    }

    /// Parse and execute one line; append successful writes to the AOF.
    pub fn handle_line(&self, line: &str) -> String {
        // SOLUTION-START
        let cmd = match parse_command(line) {
            Ok(c) => c,
            Err(e) => return format!("ERR {e}"),
        };
        let reply = self.store.lock().unwrap().apply(&cmd, Instant::now());
        if let (true, Some(aof)) = (cmd.is_write() && !reply.starts_with("ERR"), &self.aof) {
            let mut f = aof.lock().unwrap();
            let _ = writeln!(f, "{}", line.trim()).and_then(|_| f.flush());
        }
        reply
        // SOLUTION-END
    }
}

/// Accept connections forever, one task per client.
pub async fn serve(listener: TcpListener, server: Arc<Server>) -> io::Result<()> {
    // SOLUTION-START
    loop {
        let (stream, _) = listener.accept().await?;
        let server = Arc::clone(&server);
        tokio::spawn(async move {
            let _ = handle_connection(stream, server).await;
        });
    }
    // SOLUTION-END
}

// SOLUTION-ONLY-START
async fn handle_connection(stream: TcpStream, server: Arc<Server>) -> io::Result<()> {
    let (r, mut w) = stream.into_split();
    let mut lines = BufReader::new(r).lines();
    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }
        let reply = server.handle_line(&line);
        w.write_all(format!("{reply}\n").as_bytes()).await?;
    }
    Ok(())
}
// SOLUTION-ONLY-END

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
