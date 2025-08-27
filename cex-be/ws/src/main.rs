use std::{collections::{HashMap, HashSet}, env, net::SocketAddr, sync::Arc};
use futures::{lock::Mutex, SinkExt, StreamExt};
use tokio::{net::{TcpListener, TcpStream}, sync::broadcast};
use log::{info, error};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use redis::AsyncCommands;



// similiar to this const subscriptions = { clientId: { ws, rooms: [] } }
type ClientId = uuid::Uuid;

struct Subscription {
    tx: tokio::sync::mpsc::UnboundedSender<Message>,
    rooms: HashSet<String>
}

type Subscriptions = Arc<Mutex<HashMap<ClientId, Subscription>>>;


#[tokio::main]
async fn main() {
    env_logger::init();
    let (tx, _rx) = broadcast::channel::<String>(16);

    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8000".to_string());
    let addr: SocketAddr = addr.parse().expect("Invalid Address");

    let listenter = TcpListener::bind(&addr).await.expect("Failed to bind");

    info!("Listening to addr: {}", addr);
    println!("Listening to addr: {}", addr);

    let subscriptions: Subscriptions = Arc::new(Mutex::new(HashMap::new()));

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let redis_client = Arc::new(redis::Client::open(redis_url).expect("invalid REDIS_URL"));


    while let Ok((stream, _)) = listenter.accept().await {
        let subscriptions = Arc::clone(&subscriptions);
        let redis_client = Arc::clone(&redis_client);
        tokio::spawn(handle_connection(stream, tx.clone(), subscriptions, redis_client));
    }
}

async fn handle_connection(stream: TcpStream, tx: broadcast::Sender<String>, _subscriptions: Subscriptions, redis_client: Arc<redis::Client>) {
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
    let (mut ws_sink, mut receiver) = ws_stream.split();

    // Outbound queue to serialize all writes to the socket
    let (out_tx, mut out_rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

    // Task: drain outbound queue into the websocket sink
    tokio::spawn(async move {
        while let Some(msg) = out_rx.recv().await {
            if ws_sink.send(msg).await.is_err() {
                break;
            }
        }
    });

    let mut rx = tx.subscribe();
    let out_tx_clone = out_tx.clone();
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if out_tx_clone.send(Message::Text(msg.into())).is_err() {
                break;
            }
        }
    });

    println!("reciever here {:?}", receiver);
    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                                    println!("Message here: {}", text);

                                    let parsed: serde_json::Value = match serde_json::from_str(&text) {
                                        Ok(val) => val,
                                        Err(_) => {
                                            eprintln!("Invalid JSON received: {}", text);
                                            continue;
                                        }
                                    };
                                    if let Some(msg_type) = parsed.get("type").and_then(|v| v.as_str()) {
                                        match msg_type {
                                            "SUBSCRIBE" => {
                                                if let Some(room_id) = parsed.get("roomId").and_then(|v| v.as_str()) {
                                                    println!("Client subscribed to {}", room_id);

                                                    // Create a dedicated pubsub for this subscription
                                                    let client = Arc::clone(&redis_client);
                                                    let room = room_id.to_string();
                                                    let out_tx_pub = out_tx.clone();

                                                    tokio::spawn(async move {
                                                        let conn = client.as_ref().get_async_connection().await;
                                                        let Ok(conn) = conn else { return; };
                                                        let mut pubsub = conn.into_pubsub();
                                                        if pubsub.subscribe(room.as_str()).await.is_err() { return; }

                                                        let mut stream = pubsub.on_message();
                                                        while let Some(msg) = stream.next().await {
                                                            let payload: String = match msg.get_payload() {
                                                                Ok(p) => p,
                                                                Err(_) => continue,
                                                            };
                                                            if out_tx_pub.send(Message::Text(payload.into())).is_err() {
                                                                break;
                                                            }
                                                        }
                                                    });
                                                }
                                            }
                                            "sendMessage" => {
                                                if let (Some(room_id), Some(message)) = (
                                                    parsed.get("roomId").and_then(|v| v.as_str()),
                                                    parsed.get("message").and_then(|v| v.as_str()),
                                                ) {
                                                    println!("Publishing message to room {}: {}", room_id, message);

                                                    // Build JSON payload and publish to Redis
                                                    let payload = serde_json::json!({
                                                        "roomId": room_id,
                                                        "message": message
                                                    }).to_string();

                                                    let client = Arc::clone(&redis_client);
                                                    let channel = room_id.to_string();
                                                    tokio::spawn(async move {
                                                        let conn = client.as_ref().get_async_connection().await;
                                                        let Ok(mut conn) = conn else { return; };
                                                        let _: Result<(), _> = conn.publish(channel.as_str(), payload).await;
                                                    });
                                                }
                                            }
                                            _ => {
                                                println!("Unknown message type: {}", msg_type);
                                            }
                                        }
                                    }
                                }
            Ok(Message::Binary(_)) => {}
            Ok(Message::Ping(_)) => {}
            Ok(Message::Pong(_)) => {}
            Ok(Message::Close(_)) => break,
            Ok(Message::Frame(_)) => {}
            Err(_) => info!("Error it is my boy"),
        }
    }
}