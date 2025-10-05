use std::{ env, net::SocketAddr};
use tokio::net::{TcpListener, TcpStream};
use log::{info, warn, error};
use futures::{StreamExt, SinkExt};
use tokio_tungstenite::accept_async;

mod user;
pub mod types;
mod subscription_manager;
mod user_manager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    dotenvy::dotenv().ok();

    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8000".to_string());
    let addr: SocketAddr = addr.parse().expect("Invalid Address");

    let listenter = TcpListener::bind(&addr).await.expect("Failed to bind");

    info!("Starting CEX WebSocket server...");
    info!("Listening on address: {}", addr);
    info!("WebSocket server is ready to accept connections");

    while let Ok((stream, _)) = listenter.accept().await {
        info!("New WebSocket connection established");
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream) {
    let mut ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("WebSocket handshake failed: {}", e);
            return;
        }
    };
    
    info!("WebSocket connection established successfully");

    while let Some(result) = ws_stream.next().await {
        match result {
            Ok(msg) => {
                if msg.is_text() || msg.is_binary() {
                    if ws_stream.send(msg).await.is_err() { 
                        warn!("Failed to send message to client");
                        break; 
                    }
                } else if msg.is_close() {
                    info!("Client disconnected gracefully");
                    break;
                }
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
        }
    }
    
    info!("WebSocket connection closed");
}