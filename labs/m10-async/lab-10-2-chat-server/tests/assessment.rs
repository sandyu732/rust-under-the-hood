use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::time::timeout;

struct Client {
    lines: Lines<BufReader<OwnedReadHalf>>,
    writer: OwnedWriteHalf,
}

impl Client {
    async fn join(addr: std::net::SocketAddr, name: &str) -> Client {
        let (r, w) = TcpStream::connect(addr).await.unwrap().into_split();
        let mut c = Client {
            lines: BufReader::new(r).lines(),
            writer: w,
        };
        c.say(name).await;
        assert_eq!(c.recv().await, format!("* welcome {name}"));
        c
    }
    async fn say(&mut self, text: &str) {
        self.writer
            .write_all(format!("{text}\n").as_bytes())
            .await
            .unwrap();
    }
    async fn recv(&mut self) -> String {
        timeout(Duration::from_secs(3), self.lines.next_line())
            .await
            .expect("timed out waiting for a line")
            .unwrap()
            .expect("connection closed")
    }
}

async fn start() -> std::net::SocketAddr {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(lab_10_2_chat_server::run(listener));
    addr
}

#[tokio::test]
async fn messages_reach_other_clients() {
    let addr = start().await;
    let mut alice = Client::join(addr, "alice").await;
    let mut bob = Client::join(addr, "bob").await;
    let mut cara = Client::join(addr, "cara").await;
    alice.say("hi all").await;
    assert_eq!(bob.recv().await, "alice: hi all");
    assert_eq!(cara.recv().await, "alice: hi all");
    bob.say("hey").await;
    assert_eq!(
        alice.recv().await,
        "bob: hey",
        "alice should not get her own message first"
    );
}

#[tokio::test]
async fn leaving_is_announced_and_server_survives() {
    let addr = start().await;
    let mut alice = Client::join(addr, "alice").await;
    let bob = Client::join(addr, "bob").await;
    drop(bob);
    assert_eq!(alice.recv().await, "* bob left");
    let mut dan = Client::join(addr, "dan").await;
    dan.say("still up?").await;
    assert_eq!(alice.recv().await, "dan: still up?");
}
