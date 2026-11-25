use capstone_b_mini_redis::*;
use std::time::{Duration, Instant};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[test]
fn parses_commands() {
    assert_eq!(parse_command("ping"), Ok(Command::Ping));
    assert_eq!(parse_command("GET k"), Ok(Command::Get("k".into())));
    assert_eq!(
        parse_command("set k v ex 10"),
        Ok(Command::Set {
            key: "k".into(),
            value: "v".into(),
            ttl: Some(Duration::from_secs(10))
        })
    );
    assert_eq!(
        parse_command("DEL a b"),
        Ok(Command::Del(vec!["a".into(), "b".into()]))
    );
    assert!(parse_command("GET").is_err());
    assert!(parse_command("SET k v EX zero").is_err());
    assert!(
        parse_command("FLY away")
            .unwrap_err()
            .contains("unknown command")
    );
    assert!(parse_command("").is_err());
}

#[test]
fn store_semantics_and_ttl() {
    let mut s = Store::new();
    let t0 = Instant::now();
    let run = |s: &mut Store, line: &str, t: Instant| s.apply(&parse_command(line).unwrap(), t);
    assert_eq!(run(&mut s, "GET a", t0), "(nil)");
    assert_eq!(run(&mut s, "SET a 1", t0), "OK");
    assert_eq!(run(&mut s, "INCR a", t0), "(integer) 2");
    assert_eq!(run(&mut s, "INCR fresh", t0), "(integer) 1");
    assert_eq!(run(&mut s, "SET word hi", t0), "OK");
    assert_eq!(run(&mut s, "INCR word", t0), "ERR value is not an integer");
    assert_eq!(run(&mut s, "SET tmp x EX 5", t0), "OK");
    assert_eq!(run(&mut s, "GET tmp", t0 + Duration::from_secs(4)), "x");
    assert_eq!(run(&mut s, "GET tmp", t0 + Duration::from_secs(5)), "(nil)");
    assert_eq!(
        run(&mut s, "DEL a tmp missing", t0 + Duration::from_secs(6)),
        "(integer) 1"
    );
}

async fn start(server: std::sync::Arc<Server>) -> std::net::SocketAddr {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(serve(listener, server));
    addr
}

async fn send(addr: std::net::SocketAddr, lines: &[&str]) -> Vec<String> {
    let (r, mut w) = TcpStream::connect(addr).await.unwrap().into_split();
    let mut reader = BufReader::new(r).lines();
    let mut replies = Vec::new();
    for l in lines {
        w.write_all(format!("{l}\n").as_bytes()).await.unwrap();
        replies.push(reader.next_line().await.unwrap().unwrap());
    }
    replies
}

#[tokio::test]
async fn over_tcp() {
    let addr = start(Server::open(None).unwrap()).await;
    assert_eq!(
        send(addr, &["PING", "SET k v", "GET k", "bogus"]).await[..3],
        ["PONG", "OK", "v"]
    );
    assert!(send(addr, &["bogus"]).await[0].starts_with("ERR"));
}

#[tokio::test]
async fn many_concurrent_clients() {
    let addr = start(Server::open(None).unwrap()).await;
    let tasks: Vec<_> = (0..200)
        .map(|_| tokio::spawn(send(addr, &["INCR hits"])))
        .collect();
    for t in tasks {
        t.await.unwrap();
    }
    assert_eq!(send(addr, &["GET hits"]).await, ["200"]);
}

#[tokio::test]
async fn data_survives_restart() {
    let dir = tempfile::tempdir().unwrap();
    let aof = dir.path().join("data.aof");
    {
        let server = Server::open(Some(&aof)).unwrap();
        server.handle_line("SET name ferris");
        server.handle_line("INCR visits");
        server.handle_line("INCR visits");
        server.handle_line("SET gone x");
        server.handle_line("DEL gone");
        server.handle_line("GET name"); // reads are not logged
    }
    let restarted = Server::open(Some(&aof)).unwrap();
    assert_eq!(restarted.handle_line("GET name"), "ferris");
    assert_eq!(restarted.handle_line("GET visits"), "2");
    assert_eq!(restarted.handle_line("GET gone"), "(nil)");
    let log = std::fs::read_to_string(&aof).unwrap();
    assert!(!log.contains("GET"), "only writes belong in the AOF");
}
