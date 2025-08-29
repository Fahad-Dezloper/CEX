use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutgoingMessage {
    pub event: String,
    pub data: String
}


#[derive(Deserialize)]
pub struct IncomingMessage {
    pub method: String,
    pub params: Vec<String>,
}
