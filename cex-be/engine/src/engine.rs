use std::collections::HashMap;

use crate::{orderbook::{self, OrderBook}, types::{ProcessInput, Side}};

pub struct UserBalance {
    available: f64,
    locked: f64
}

pub struct Engine{
    orderbook: Vec<OrderBook>,
    balance: HashMap<String, UserBalance>
}

pub struct Order {
    price: f64,
    quantity: f64,
    order_id: String,
    filled: f64,
    side: Side,
    user_id: String
}


impl Engine { 
    pub fn new() -> Self {
        println!("wassup my man");
        Self {
            orderbook: Vec::new(),
            balance: HashMap::new()
        }
    }

    pub fn process(&mut self, msg: ProcessInput){
        println!("wassup mf {}", msg.client_id);
        match &msg.message {
            crate::types::MessageFromApi::CREATE_ORDER(_) => {
                // call create order function
                let (executed_qty, fills, order_id) = self.create_order("BTC-USD", 50000.0, 10, "buy", "user123");

                println!("Executed Qty: {}", executed_qty);
                println!("Fills: {:?}", fills);
                println!("Order ID: {}", order_id);
                // send  to redis manager
                // if error send to redis order cancelled
            },
            crate::types::MessageFromApi::CANCEL_ORDER(_) => {
                // cancel order function
                if let crate::types::MessageFromApi::CANCEL_ORDER(cancel_data) = &msg.message {
                    println!("Cancel order: order_id = {}, market = {}", cancel_data.order_id, cancel_data.market);
                }
                // send to redis manager
            },
            crate::types::MessageFromApi::ON_RAMP(_) => {
                // call on ramp fuction
                if let crate::types::MessageFromApi::ON_RAMP(ramp_data) = &msg.message {
                    println!("Ramp data Amount: {}, user_id: {}, txn_id: {}", ramp_data.amount, ramp_data.user_id, ramp_data.txn_id)
                }
                // break
            },
            crate::types::MessageFromApi::GET_DEPTH(_) => {
                // get orderbook depth and send to redis
                if let crate::types::MessageFromApi::GET_DEPTH(depth_data) = &msg.message {
                    println!("Market: {}", depth_data.market)
                }
            },
            crate::types::MessageFromApi::GET_OPEN_ORDERS(_) => {
                // get open orders
                if let crate::types::MessageFromApi::GET_OPEN_ORDERS(open_order_data) = &msg.message {
                    println!("User Id: {} Market: {}", open_order_data.user_id, open_order_data.market);
                }
                // send to redis
            },
        };
        println!("wassup");
    }

    pub fn create_order(
        &mut self,
        market: &str,
        price: f64,
        quantity: u64,
        side: &str,
        user_id: &str,
    ) -> (u64, Vec<String>, String) {
        let orderbook = self.orderbooks
                            .iter()
                            .find(|o| o.ticker() == market)
                            .ok_or_else(|| "No orderbook found".to_string());

        // do check and lock funds


        println!(
            "Creating order: market={}, price={}, quantity={}, side={}, user_id={}",
            market, price, quantity, side, user_id
        );
        (executed_qty, fills, order_id)
    }

    // check and lock funds
    pub fn check_and_lock_funds(baseAsset: String, quoteAsset: String, side: Side, user_id: String, asset: String, price: String, quantity: u64) {
        if side == Side::Buy {
            // Assuming self.balances: HashMap<String, HashMap<String, Balance>>
            // and Balance { available: f64, locked: f64 }
            let user_balances = self.balances.get_mut(&user_id).expect("User not found");
            let quote_balance = user_balances.get_mut(&quoteAsset).expect("Quote asset not found");
            let price_f64: f64 = price.parse().expect("Invalid price");
            let required = quantity as f64 * price_f64;
            if quote_balance.available < required {
                panic!("Insufficient funds");
            }
            quote_balance.available -= required;
            quote_balance.locked += required;
        } else {
            let user_balances = self.balances.get_mut(&user_id).expect("User not found");
            let base_balance = user_balances.get_mut(&baseAsset).expect("Base asset not found");
            if base_balance.available < quantity as f64 {
                panic!("Insufficient funds");
            }
            base_balance.available -= quantity as f64;
            base_balance.locked += quantity as f64;
        }
    }
}