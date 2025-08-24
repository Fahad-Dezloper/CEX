use std::{collections::HashMap, sync::Arc};

use log::info;
use poem::web::Data;
use serde::{Deserialize, Serialize};

use crate::{orderbook::{Fill, Orderbook}, redis::redis_manager::RedisManager, types::{CreateOrderData, EngineData, MessageType, Process, Side, UserBalance}};

pub const BASE_CURRENCY: &str = "INR";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Engine {
    pub orderbook: Vec<Orderbook>,
    balances: HashMap<String, HashMap<String, UserBalance>>
}

impl Engine {
    pub fn new() -> Self {
        info!("Initializing engine with SOL_USDC orderbook");
        let mut engine = Self {
            orderbook: vec![
                Orderbook::new("SOL_USDC".to_string()),
            ],
            balances: HashMap::new()
        };

        engine.set_base_balances();
        info!("Engine initialized with orderbooks: {:?}", engine.orderbooks.iter().map(|ob| ob.ticker()).collect::<Vec<_>>());
        engine
    }

    pub fn process(&self, msg: Process) {
        println!("Processing message: {:?} {:?}", msg.client_id, msg.message);
        
        match msg.type_ {
            MessageType::CreateOrder => self.handle_create_order(msg.message),
            MessageType::CancelOrder => self.handle_cancel_order(msg.message),
            MessageType::GetOpenOrders => self.handle_get_open_orders(msg.message),
            MessageType::OnRamp => self.handle_on_ramp(msg.message),
            MessageType::GetDepth => self.handle_get_depth(msg.message),
        }
    }

    fn set_base_balances(&mut self) {
        // Remove hardcoded user IDs
        // self.balances is now empty by default and will be populated as needed
    }

    // create order and other function will go to orderbook.rs
    fn handle_create_order(&self, data: EngineData) {
        println!("CREATE Order");
        if let EngineData::Order(order_data) = data {
            println!("Creating order: at price {} {}", order_data.price, order_data.market);
            // Your create order logic here
            let order = create_order(order_data.market, order_data.price, order_data.quantity, order_data.side, order_data.user_id);

        } else {
            println!("Invalid data for CreateOrder");
        }
    }

    fn handle_cancel_order(&self, data: EngineData) {
        println!("CANCEL Order");
        if let EngineData::DeleteOrder(cancel_data) = data {
            println!("Cancelling order: {} in market {}", 
                cancel_data.order_id, cancel_data.market);
            // Your cancel order logic here
        }
    }

    fn handle_get_open_orders(&self, data: EngineData) {
        println!("GET_OPEN_ORDERS");
        if let EngineData::OpenOrder(open_order_data) = data {
            println!("Getting open orders for user: {} in market {}", 
                open_order_data.user_id, open_order_data.market);
            // Your get open orders logic here
        }
    }

    fn handle_on_ramp(&self, data: EngineData) {
        println!("ON_RAMP");
        // Your on-ramp logic here
    }

    fn handle_get_depth(&self, data: EngineData) {
        println!("GET_DEPTH");
        if let EngineData::Symbol(symbol_data) = data {
            println!("Getting depth for market: {}", symbol_data.market);
            // Your get depth logic here
        }
    }



    // real logic functions
    pub fn create_order(
        &mut self,
        msg: CreateOrderData
    ) -> Result<(f64, Vec<Fill>, String)> {
        info!("Creating order for market: {:?}", msg.market)
    }
}