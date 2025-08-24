use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Orderbook {
    market: String,
    pub bids: BTreeMap<Price, Vec<Order>>,
    pub asks: BTreeMap<Price, Vec<Order>>,
    last_trade_id: i64,
    current_price: f64,
    orders: HashMap<String, Order>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fill {
    pub qty: f64,
    pub price: f64,
    pub trade_id: i64,
    pub marker_order_id: String,
    pub other_user_id: String,
}