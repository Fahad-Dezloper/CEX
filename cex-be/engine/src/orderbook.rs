use std::collections::{BTreeMap, HashMap};
use log::info;
use serde::{Deserialize, Serialize};

use crate::types::{CreateOrderData, DeleteOrderData, GetDepth, GetOpenOrder};

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

impl Orderbook {
    pub fn new(market: String) -> Self {
        Self {
            market,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            last_trade_id: 0,
            current_price: 0.0,
            orders: HashMap::new(),
        }
    }

    pub fn ticker(&self) -> &str {
        &self.market
    }

    // Logic functions moved from main_engine.rs
    pub fn create_order(&mut self, msg: &CreateOrderData) -> Result<(f64, Vec<Fill>, String), String> {
        info!("Creating order for market: {:?}", msg.market);
        // Placeholder logic: return dummy values for now
        Ok((0.0, Vec::new(), String::from("order_id_placeholder")))
    }

    pub fn cancel_order(&mut self, msg: &DeleteOrderData) -> Result<(), String> {
        info!("Alot of logic needs to happen here");
        Ok(())
    }

    pub fn open_order(&mut self, msg: &GetOpenOrder) -> Result<(), String> {
        info!("Not too much logic needs to happen");
        Ok(())
    }

    pub fn get_depth(&mut self, msg: &GetDepth) -> Result<(Vec<(f64, f64)>, Vec<(f64, f64)>), String> {
        info!("Just give back bids and asks");
        // Placeholder: return empty vectors for bids and asks
        Ok((Vec::new(), Vec::new()))
    }
}