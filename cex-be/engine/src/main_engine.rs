use std::{collections::HashMap};
use log::info;
use serde::{Deserialize, Serialize};

use crate::{orderbook::{Fill, Orderbook}, redis::redis_manager::RedisManager, types::{CreateOrderData, DeleteOrderData, EngineData, GetDepth, GetOpenOrder, MessageType, Process, UserBalance}};

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

    pub fn process(&mut self, msg: Process) {
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
    fn handle_create_order(&mut self, data: EngineData) {
        println!("CREATE Order");
        match data {
            EngineData::Order(order_data) => {
                println!("Creating order: at price {} {}", order_data.price, order_data.market);
                match self.create_order(&order_data) {
                    Ok((executed_qty, fills, order_id)) => {
                        // Here you would send a success message to the API
                        // RedisManager::get_instance().lock().unwrap().send_to_api(
                        //     &order_data.user_id,
                        //     MessageToApi::OrderPlaced {
                        //         order_id,
                        //         executed_qty,
                        //         fills
                        //     }
                        // ).unwrap();
                        println!("Order created successfully: {}, qty: {}, fills: {:?}", order_id, executed_qty, fills);
                    }
                    Err(e) => {
                        // Here you would send an error message to the API
                        // RedisManager::get_instance().lock().unwrap().send_to_api(
                        //     &order_data.user_id,
                        //     MessageToApi::OrderError {
                        //         error: e.clone()
                        //     }
                        // ).unwrap();
                        println!("Failed to create order: {}", e);
                    }
                }
            }
            _ => {
                println!("Invalid data for CreateOrder");
            }
        }
    }

    fn handle_cancel_order(&self, data: EngineData) {
        println!("CANCEL Order");
        match data {
            EngineData::DeleteOrder(delete_order_data) => {
                println!("Cancel Order: of price {} {}", delete_order_data.market, delete_order_data.order_id);
                match self.cancel_order(&delete_order_data){
                    Ok((order_id, executed_qty, remain_qty)) => {
                        // Here you would send a success message to the API
                        // RedisManager::get_instance().lock().unwrap().send_to_api(
                        //      &delete_order_data.order_id,
                        // MessageToApi::OrderCancelled {
                        //     order_id,
                        //     executed_qty,
                        //     remain_qty
                        // } 
                        // ).unwrap()
                        println!("Order Cancelled Successfully: {}", orderDeleted)
                    }
                    Err(e) => {
                        println!("Failed to Cancel Order: {}", e);
                    }
                }
            }
            _=> {
                println!("Invalid data for delete order")
            }
        }
    }

    fn handle_get_open_orders(&self, data: EngineData) {
        println!("GET_OPEN_ORDERS");
        match data {
            EngineData::OpenOrder(open_order_data) => {
                println!("Getting Open Orders");
                match self.open_order(&open_order_data) {
                    Ok(()) => {
                        println!("Order Opened successfully")
                    }
                    Err(e) => {
                        println!("Failed to fetch Open Orders")
                    }
                }
            }
            _ => {
                println!("Invalid data for open orders")
            }
        }
    }

    fn handle_on_ramp(&self, data: EngineData) {
        // Your on-ramp logic here

        // add type in types.rs file follow the same structure

        // println!("ON_RAMP");
        // match data {
        //     Engin
        // }
    }

    fn handle_get_depth(&self, data: EngineData) {
        println!("GET_DEPTH");
        match data {
            EngineData::Depth(market_data) => {
                println!("Market data is here: {:?}", market_data.market);
                match self.get_depth(&market_data) {
                    Ok((bids, asks)) => {
                        println!("Returning bids and asks: bids = {:?}, asks = {:?}", bids, asks);
                        // TODO: Call function to send depth via redisManager
                    }
                    Err(e) => {
                        println!("Failed to get Depth")
                    }
                }
            }
            _=> {
                println!("Invalid market data recieved")
            }
        }
        }








    // real logic functions
    pub fn create_order(
        &mut self,
        msg: &CreateOrderData
    ) -> Result<(f64, Vec<Fill>, String), String> {
        info!("Creating order for market: {:?}", msg.market);
        // Placeholder logic: return dummy values for now
        Ok((0.0, Vec::new(), String::from("order_id_placeholder")))
    }

    pub fn cancel_order(&mut self, msg: &DeleteOrderData) -> Result<()> {
        info!("Alot of logic needs to happen here");
    }

    pub fn open_order(&mut self, msg: &GetOpenOrder) -> Result<()> {
        info!("Not too much logic needs to happen")
    }

    pub fn get_depth(&mut self, msg: &GetDepth) -> Result<()> {
        info!("Just give back bids and asks")
    }
}