use std::{env, net::SocketAddr};
use futures::{SinkExt, StreamExt};
use tokio::{net::{TcpListener, TcpStream}, sync::broadcast};
use log::{info, error};
use tokio_tungstenite::{accept_async, tungstenite::Message};



#[tokio::main]
async fn main() {
    env_logger::init();
    let (tx, _rx) = broadcast::channel::<String>(16);

    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8000".to_string());
    let addr: SocketAddr = addr.parse().expect("Invalid Address");

    let listenter = TcpListener::bind(&addr).await.expect("Failed to bind");

    info!("Listening to addr: {}", addr);
    println!("Listening to addr: {}", addr);

    while let Ok((stream, _)) = listenter.accept().await {
        tokio::spawn(handle_connection(stream, tx.clone()));
    }
}

async fn handle_connection(stream: TcpStream, tx: broadcast::Sender<String>) {
    println!("stream here {:?}", stream);
    // Accept the WebSocket connection
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("Error during the websocket handshake: {}", e);
            return;
        }
    };

    // Split the WebSocket stream into a sender and receiver
    let (mut sender, mut receiver) = ws_stream.split();

    let mut rx = tx.subscribe();
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    println!("reciever here {:?}", receiver);
    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("MEssage here {}", Message::Text(text.clone()));
                let reversed = text.chars().collect::<String>();
                // if let Err(e) = sender.send(Message::Text(reversed.into())).await {
                //     error!("Error sending message: {}", e);
                // }
                let _ = tx.send(reversed.clone());  
            }
            Ok(Message::Close(_)) => break,
            Ok(_) => (),
            Err(e) => {
                error!("Error processing message: {}", e);
                break;
            }
        }
    }
}