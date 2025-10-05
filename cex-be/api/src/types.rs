use serde::{Deserialize, Serialize};
use time::Time;
use validator::Validate;


#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")] 
pub enum Side {
    Buy,
    Sell
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateOrder {
    #[validate(length(min = 7, max = 20))]
    pub market: String,
    #[validate(range(min = 0.01))]
    pub price: f64,
    #[validate(range(min = 0.00000001))]
    pub quantity: f64,
    pub side: Side,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteOrder {
    pub order_id: String,
    pub market: String
}

// market, price, quantity, side, userId

#[derive(Debug, Serialize, Deserialize)]
pub struct DepthQuery {
    pub symbol: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KlinesQuery {
    pub market: String,
    pub interval: String,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "endTime")]
    pub end_time: i64
}


#[derive(Serialize, Deserialize)]
pub struct SymbolData {
    pub market: String
}

#[derive(Serialize, Deserialize)]
pub struct OpenMarketRequest {
    pub user_id: String,
    pub market: String
}

#[derive(Serialize, Deserialize)]
pub struct CreateOrderData {
    pub market: String,
    pub price: f64,
    pub quantity: f64,
    pub side: Side,
    pub user_id: String
}

#[derive(Serialize, Deserialize)]
pub struct DeleteOrderData {
    pub market: String,
    pub order_id: String
}

#[derive(Serialize, Deserialize)]
pub struct GetOpenOrder {
    pub user_id: String,
    pub market: String
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum EngineData {
    Symbol(SymbolData),
    Order(CreateOrderData),
    DeleteOrder(DeleteOrderData),
    OpenOrder(GetOpenOrder)
    // Future variants can be added here, e.g. Order(OrderData), etc.
}

#[derive(Serialize, Deserialize)]
pub struct MessageToEngine {
    #[serde(rename = "type")]
    pub type_ : String,
    pub data: EngineData
}

// klines return data
#[derive(Serialize, Deserialize)]
pub struct KlinesData {
    pub close: f64,
    pub end: String,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub quote_volume: f64,
    pub start: String,
    pub trades: f64,
    pub volume: f64,
}
