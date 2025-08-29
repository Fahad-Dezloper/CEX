use tokio_tungstenite::{WebSocketStream, tungstenite::Message};
use tokio::net::TcpStream;
use futures::{SinkExt, StreamExt};
use crate::types::{IncomingMessage, OutgoingMessage};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::subscription_manager::SubscriptionManager;

pub struct User {
    id: String,
    ws: WebSocketStream<TcpStream>,
    subscription: Vec<String>,
    subscription_manager: Arc<Mutex<SubscriptionManager>>
}

pub const SUBSCRIBE: &str = "SUBSCRIBE";
pub const UNSUBSCRIBE: &str = "UNSUBSCRIBE";

impl User {
    pub fn new(
        id: impl Into<String>,
        ws: WebSocketStream<TcpStream>,
        subscription_manager: Arc<Mutex<SubscriptionManager>>,
    ) -> Self {
        Self { 
            id: id.into(), 
            ws, 
            subscription: Vec::new(),
            subscription_manager
        }
    }

    pub async fn subscribe(&mut self, subscription: String) {
        self.subscription.push(subscription.clone());

        let mut manager = self.subscription_manager.lock().await;
        manager.subscribe(self.id.clone(), subscription).await.unwrap_or_else(|e| {
            eprintln!("Failed to subscribe: {}", e);
        });
    }
    
    pub async fn unsubscribe(&mut self, subscription: String) {
        self.subscription.retain(|s| s.to_string() != subscription);

        let mut manager = self.subscription_manager.lock().await;
        manager.unsubscribe(self.id.clone(), subscription.to_string()).await.unwrap_or_else(|e| {
            eprintln!("Failed to unsubscribe: {}", e);
        });
    }

    pub async fn emit(&mut self, message: OutgoingMessage) -> anyhow::Result<()> {
        let json = serde_json::to_string(&message)?;
        self.ws.send(Message::Text(json.into())).await.map_err(|e| anyhow::anyhow!("WebSocket send error: {}", e))?;
        Ok(())
    }

    pub async fn listen(mut self) {
        while let Some(msg) = self.ws.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(parsed) = serde_json::from_str::<IncomingMessage>(&text) {
                        match parsed.method.as_str() {
                            SUBSCRIBE => {
                                for s in parsed.params {
                                    self.subscribe(s).await;
                                }
                            }
                            UNSUBSCRIBE => {
                                for s in parsed.params {
                                    self.unsubscribe(s).await;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Websocket error for user {}: {:?}", self.id, e);
                    break;
                }
            }
        }
    }
}