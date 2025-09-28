use std::collections::HashMap;

use uuid::Uuid;

use crate::{orderbook::OrderBook, types::{ProcessInput, Side}};

pub struct UserBalance {
    available: f64,
    locked: f64
}

pub struct Engine{
    orderbooks: Vec<OrderBook>,
    balances: HashMap<String, HashMap<String, UserBalance>>
}

pub struct Order {
    pub price: f64,
    pub quantity: f64,
    pub order_id: String,
    pub filled: f64,
    pub side: Side,
    pub user_id: String
}


impl Engine { 
    pub fn new() -> Self {
        println!("wassup my man");
        Self {
            orderbooks: Vec::new(),
            balances: HashMap::new()
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
        // Find orderbook index first to avoid borrowing conflicts
        let orderbook_index = match self.orderbooks
                            .iter()
                            .position(|o| o.ticker() == market) {
            Some(index) => index,
            None => return (0, Vec::new(), "No orderbook found".to_string())
        };

        if let Some((base, quote)) = market.split_once('-') {
            println!("Base: {}", base);   // BTC
            println!("Quote: {}", quote); // USD

             // Store them individually
            let btc = base.to_string();
            let usd = quote.to_string();

            println!("btc = {}, usd = {}", btc, usd);
        }

        let side_enum = match side {
            "buy" => Side::Buy,
            "sell" => Side::Sell,
            _ => return (0, Vec::new(), "Invalid side".to_string())
        };

        // do check and lock funds
        if let Some((base, quote)) = market.split_once('-') {
            self.check_and_lock_funds(base.to_string(), quote.to_string(), side_enum.clone(), user_id.to_string(), price.to_string(), quantity);
        }

        let order = Order { 
            price: price, 
            quantity: quantity as f64, 
            order_id: Uuid::new_v4().to_string(), 
            filled: 0.0, 
            side: side_enum, 
            user_id: user_id.to_string() 
        };

        let (executed_qty, fills) = self.orderbooks[orderbook_index].addOrder(order);

        println!(
            "Creating order: market={}, price={}, quantity={}, side={}, user_id={}",
            market, price, quantity, side, user_id
        );
        (0, Vec::new(), "order_id".to_string())
    }

    // check and lock funds
    // baseAsset = "BTC" quoteAsset = "USDC" side = "buy" price = "20000" quantity = "0.5" userId = "u1"
    pub fn check_and_lock_funds(&mut self, baseAsset: String, quoteAsset: String, side: Side, user_id: String, price: String, quantity: u64) {
        if side == Side::Buy {
            // self.balances: HashMap<String, HashMap<String, Balance>>
            // and Balance { available: f64, locked: f64 }
            let user_balances = self.balances.get_mut(&user_id).expect("User not found");
            let quote_balance = user_balances.get_mut(&quoteAsset).expect("Quote asset not found");
            let price_f64: f64 = price.parse().expect("Invalid price");

            // balances.get("u1") = {
            //     USDT: { available: 15000, locked: 0 },
            //     BTC: { available: 2, locked: 0 }
            //  }
            // Required funds = quantity * price = 0.5 * 20000 = 10000 USDT
            let required = quantity as f64 * price_f64;
            if quote_balance.available < required {
                panic!("Insufficient funds");
            }

            // USDT.available = 15000 - 10000 = 5000
            quote_balance.available -= required;
            // USDT.locked = 0 + 10000 = 10000
            quote_balance.locked += required;
        } else {
            let user_balances = self.balances.get_mut(&user_id).expect("User not found");
            let base_balance = user_balances.get_mut(&baseAsset).expect("Base asset not found");
            // Check if user has enough BTC (base asset)
            // {
            //    USDT: { available: 5000, locked: 10000 },
            //    BTC: { available: 2, locked: 0 }
            // }
            if base_balance.available < quantity as f64 {
                panic!("Insufficient funds");
            }

            // BTC.available = 2 - 0.5 = 1.5
            base_balance.available -= quantity as f64;
            // BTC.locked = 0 + 0.5 = 0.5
            base_balance.locked += quantity as f64;
        }
    }
}