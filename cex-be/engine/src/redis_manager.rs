use redis::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum DbMessage {
    TradeAdded(TradeAdded),
    OrderUpdate(OrderUpdate)
}

#[derive(Serialize, Deserialize)]
pub struct TradeAdded {
    pub id: String,
    pub is_buyer_maker: bool,
    pub price: String,
    pub quote_quantity: String,
    pub timestamp: i64,
    pub market: String
}

#[derive(Serialize, Deserialize)]
pub struct OrderUpdate {
    pub order_id: String,
    pub exec_qty: f64,
    market: Option<String>,
    price: Option<String>,
    quantity: Option<String>,
    side: Option<Side>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Buy,
    Sell
}

pub struct RedisManager {
    redis_client: Client,
    ws_client: Client,
    db_client: Client
}

impl RedisManager {
    pub fn new() {
        //redis 1 pubsub
        //redis 2 pubsub
        //redis 3 queue/db
    }
}