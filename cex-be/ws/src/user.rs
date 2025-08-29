use std::{net::TcpStream};

use tokio_tungstenite::tungstenite::WebSocket;


pub struct User {
    id: String,
    ws: WebSocket<TcpStream>,
    subscription: Vec<String>
}


impl User {
    pub fn new(
        id: impl Into<String>,
        ws: WebSocket<TcpStream>,
    ) -> Self {
        Self { id: id.into(), ws, subscription: Vec::new() }
    }

    pub fn subscribe(&mut self, subscription: String) {
        self.subscription.push(subscription);
    }
    pub fn unsubscribe(&mut self, subscription: String) {
        self.subscription.retain(|s| s.to_string() != subscription);
    }
}