use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::io::{self, AsyncBufReadExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::task;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::WebSocketStream;

type Tx = broadcast::Sender<String>;
type Rx = broadcast::Receiver<String>;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8080".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    let (tx, _rx) = broadcast::channel::<String>(100);
    println!("WebSocket server running on ws://{}", addr);
    task::spawn(read_stdin(tx.clone()));
    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        let rx = tx.subscribe();
        task::spawn(handle_client(stream, tx, rx));
    }
}

async fn read_stdin(tx: Tx) {
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin).lines();
    while let Ok(Some(line)) = reader.next_line().await {
        if !line.is_empty() {
            let _ = tx.send(line.clone());
            println!("Sent: {}", line);
        }
    }
}

async fn handle_client(stream: TcpStream, _tx: Tx, mut rx: Rx) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(_) => return,
    };
    println!("New WebSocket connection established");
    let (mut ws_sender, ws_receiver) = ws_stream.split();
    task::spawn(handle_client_messages(ws_receiver));
    while let Ok(notification) = rx.recv().await {
        if ws_sender.send(notification.into()).await.is_err() {
            break;
        }
    }
}

async fn handle_client_messages(
    mut ws_receiver: futures::stream::SplitStream<WebSocketStream<TcpStream>>,
) {
    while let Some(msg) = ws_receiver.next().await {
        if let Ok(text) = msg.map(|m| m.to_text().unwrap_or("").to_string()) {
            println!("Received from client: {}", text);
        }
    }
}
