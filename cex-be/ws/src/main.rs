use std::{ env, net::SocketAddr};
use tokio::net::{TcpListener, TcpStream};
use log::info;
use futures::{StreamExt, SinkExt};
use tokio_tungstenite::accept_async;

mod user;
pub mod types;
mod subscription_manager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8000".to_string());
    let addr: SocketAddr = addr.parse().expect("Invalid Address");

    let listenter = TcpListener::bind(&addr).await.expect("Failed to bind");

    info!("Listening to addr: {}", addr);
    println!("Listening to addr: {}", addr);

    while let Ok((stream, _)) = listenter.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())

}

async fn handle_connection(stream: TcpStream) {
    let mut ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("WebSocket handshake failed: {}", e);
            return;
        }
    };
    println!("Websocket connection established");

    while let Some(result) = ws_stream.next().await {
        match result {
            Ok(msg) => {
                if msg.is_text() || msg.is_binary() {
                    if ws_stream.send(msg).await.is_err() { break; }
                } else if msg.is_close() {
                    println!("Client disconnected");
                    break;
                }
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }
}