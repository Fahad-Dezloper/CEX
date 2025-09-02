use crate::types::ProcessInput;


pub struct Engine{}


impl Engine { 
    pub fn new() -> Self {
        println!("wassup my man");
        Self {}
    }

    pub fn process(&self, msg: ProcessInput){
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

    fn create_order(&self, market: &str, price: f64, quantity: u32, side: &str, user_id: &str) -> (u32, Vec<String>, String) {
        (quantity, vec!["fill1".to_string()], "ORD123".to_string())
    }
}