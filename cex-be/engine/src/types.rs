use serde::{Deserialize, Serialize};

use crate::orderbook::Orderbook;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Buy,
    Sell
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageType {
    CreateOrder,
    CancelOrder,
    GetOpenOrders,
    OnRamp,
    GetDepth,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolData {
    pub market: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderData {
    pub market: String,
    pub price: f64,
    pub quantity: f64,
    pub side: Side,
    pub user_id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteOrderData {
    pub market: String,
    pub order_id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDepth {
    pub market: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOpenOrder {
    pub user_id: String,
    pub market: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EngineData {
    Symbol(SymbolData),
    Order(CreateOrderData),
    DeleteOrder(DeleteOrderData),
    OpenOrder(GetOpenOrder),
    Depth(GetDepth)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Process {
    #[serde(rename = "type")]
    pub type_: MessageType,
    pub message: EngineData,
    pub client_id: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserBalance {
    available: f64,
    locked: f64
}