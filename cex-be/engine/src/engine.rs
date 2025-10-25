use std::collections::HashMap;

use uuid::Uuid;
use log::{info, warn, error, debug};

use crate::{
    orderbook::{Fill, OrderBook, PriceLevel}, redis_manager::RedisManager, types::{DepthPayload, FillResponse, MessageToApi, OpenOrdersPayload, OrderCancelledPayload, OrderPlacedPayload, ProcessInput, PushToDb, Side, ORDERUPDATEDATA, TRADEADDEDDATA}
};

pub struct UserBalance {
    available: f64,
    locked: f64
}

pub struct Engine{
    pub orderbooks: Vec<OrderBook>,
    pub balances: HashMap<String, HashMap<String, UserBalance>>
}

#[derive(Clone)]
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
        println!("Initializing matching engine...");
        let mut engine = Self {
            orderbooks: Vec::new(),
            balances: HashMap::new()
        };
        
        // Initialize orderbooks for supported markets
        engine.initialize_markets();
        
        println!("Matching engine initialized with {} markets", engine.orderbooks.len());
        engine
    }

    fn initialize_markets(&mut self) {
        let mut initialized = false;
        if let Some(redis_manager) = RedisManager::get_instance().try_lock() {
            if let Ok(Some(json)) = redis_manager.get_cached_markets() {
                if let Ok(markets) = serde_json::from_str::<Vec<serde_json::Value>>(&json) {
                    for m in markets {
                        let base_asset = m.get("base").and_then(|v| v.as_str()).unwrap_or("");
                        let quote_asset = m.get("quote").and_then(|v| v.as_str()).unwrap_or("");
                        if base_asset.is_empty() || quote_asset.is_empty() { continue; }
                        let orderbook = OrderBook::new(
                            base_asset.to_string(),
                            quote_asset.to_string(),
                            Vec::new(),
                            Vec::new(),
                            Some(0),    
                            Some(0.0),  
                        );
                        println!("Initialized orderbook for {}-{}", base_asset, quote_asset);
                        self.orderbooks.push(orderbook);
                        initialized = true;
                    }
                }
            }
        }

        if !initialized {
            let fallback = vec![
                ("BTC", "USDC"), ("ETH", "USDC"), ("SOL", "USDC"), ("BNB", "USDC"), ("DOGECOIN", "USDC"), ("SUI", "USDC"), ("HYPERLIQUID", "USDC"),
            ];
            for (base_asset, quote_asset) in fallback {
                let orderbook = OrderBook::new(
                    base_asset.to_string(),
                    quote_asset.to_string(),
                    Vec::new(),
                    Vec::new(),
                    Some(0),    
                    Some(0.0),  
                );
                println!("Initialized orderbook for {}-{}", base_asset, quote_asset);
                self.orderbooks.push(orderbook);
            }
        }
    }

    pub fn process(&mut self, msg: ProcessInput){
        debug!("Processing message from client: {}", msg.client_id);
        match &msg.message {
            crate::types::MessageFromApi::CREATE_ORDER(create_data) => {
                let market = create_data.market.clone();
                let price: f64 = create_data.price.parse().unwrap_or(0.0);
                let quantity: u64 = create_data.quantity.parse().unwrap_or(0);
                let side = match create_data.side {
                    Side::Buy => "buy",
                    Side::Sell => "sell",
                };
                let user_id = create_data.user_id.clone();

               match self.create_order(&market, price, quantity, side, &user_id) {
                    Ok((executed_qty, fills, order_id)) => {
                        let fill_responses: Vec<FillResponse> = fills.iter().map(|fill| FillResponse {
                            price: fill.price.clone(),
                            qty: fill.qty,
                            trade_id: fill.trade_id,
                            other_user_id: fill.other_user_id.clone(),
                            market_order_id: fill.market_order_id.clone(),
                        }).collect();

                        let response = MessageToApi::ORDER_PLACED(OrderPlacedPayload {
                            order_id: order_id.clone(),
                            executed_qty,
                            fills: fill_responses,
                        });

                        if let Ok(json) = serde_json::to_string(&response) {
                            if let Some(redis_manager) = RedisManager::get_instance().try_lock() {
                                let _ = redis_manager.send_to_api(&msg.client_id, &json);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error creating order: {}", e);
                        
                        let response = MessageToApi::ORDER_CANCELLED(OrderCancelledPayload {
                            order_id: "".to_string(),
                            executed_qty: 0.0,
                            remaining_qty: 0.0,
                        });

                        if let Ok(json) = serde_json::to_string(&response) {
                            if let Some(redis_manager) = RedisManager::get_instance().try_lock() {
                                let _ = redis_manager.send_to_api(&msg.client_id, &json);
                            }
                        }
                    }
                }
            },
            crate::types::MessageFromApi::GET_DEPTH(_) => {
                if let crate::types::MessageFromApi::GET_DEPTH(depth_data) = &msg.message {
                    println!("Market: {}", depth_data.market);
                    let market = depth_data.market.clone();
                    let orderbook = self.orderbooks.iter().find(|o| o.ticker() == market).expect("Orderbook not found");

                    
                    let response = match std::panic::catch_unwind(|| orderbook.getDepth()) {
                        Ok(depth) => {
                            MessageToApi::DEPTH(DepthPayload {
                                payload: serde_json::to_string(&depth).unwrap_or("{\"bids\":[],\"asks\":[]}".to_string()),
                            })
                        }
                        Err(e) => {
                            println!("Error getting depth: {:?}", e);
                            MessageToApi::DEPTH(DepthPayload {
                                payload: "{\"bids\":[],\"asks\":[]}".to_string(),
                            })
                        }
                    };

                    if let Ok(json) = serde_json::to_string(&response) {
                        if let Some(redis_manager) = RedisManager::get_instance().try_lock() {
                            let _ = redis_manager.send_to_api(&msg.client_id, &json);
                        }
                    }
                }
            },
            crate::types::MessageFromApi::CANCEL_ORDER(_) => {
                if let crate::types::MessageFromApi::CANCEL_ORDER(cancel_data) = &msg.message {
                    let order_id = &cancel_data.order_id;
                    let cancel_market = &cancel_data.market;
                    let cancel_orderbook_index = self.orderbooks
                        .iter()
                        .position(|o| o.ticker() == cancel_market.as_str())
                        .expect("Orderbook not found");
                    
                    let (base_asset, quote_asset) = match cancel_market.split_once('-') {
                        Some((base, quote)) => (base.to_string(), quote.to_string()),
                        None => {
                            println!("Invalid market format");
                            return;
                        }
                    };
                    
                    let order_info = self.orderbooks[cancel_orderbook_index]
                        .asks
                        .iter()
                        .find(|o| o.order_id == order_id.as_str())
                        .map(|o| (o.clone(), Side::Sell))
                        .or_else(|| {
                            self.orderbooks[cancel_orderbook_index]
                                .bids
                                .iter()
                                .find(|o| o.order_id == order_id.as_str())
                                .map(|o| (o.clone(), Side::Buy))
                        });

                        if let Some((order, side)) = order_info {
                            if side == Side::Buy {
                                let price = self.orderbooks[cancel_orderbook_index].cancelBid(&order.order_id);
                                let left_qty = (order.quantity - order.filled) * order.price;

                                self.balances.get_mut(&order.user_id)
                                    .expect("User not found")
                                    .get_mut(&quote_asset)
                                    .expect("Quote asset not found")
                                    .available += left_qty;

                                self.balances.get_mut(&order.user_id)
                                    .expect("User not found")
                                    .get_mut(&quote_asset)
                                    .expect("Quote asset not found")
                                    .locked -= left_qty;

                                if let Some(price) = price {
                                    self.send_updated_depth(price.to_string(), cancel_market.to_string());
                                }
                            } else {
                                let price = self.orderbooks[cancel_orderbook_index].cancelAsk(&order.order_id);
                                let left_qty = order.quantity - order.filled;
                                self.balances.get_mut(&order.user_id)
                                    .expect("User not found")
                                    .get_mut(&base_asset)
                                    .expect("Base asset not found")
                                    .available += left_qty;

                                self.balances.get_mut(&order.user_id)
                                    .expect("User not found")
                                    .get_mut(&base_asset)
                                    .expect("Base asset not found")
                                    .locked -= left_qty;

                                if let Some(price) = price {
                                    self.send_updated_depth(price.to_string(), cancel_market.to_string());
                                }
                            }
                        } else {
                            println!("Order not found: {}", order_id);
                        }

                        let response = MessageToApi::ORDER_CANCELLED(OrderCancelledPayload {
                            order_id: order_id.to_string(),
                            executed_qty: 0.0,
                            remaining_qty: 0.0,
                        });

                        if let Ok(json) = serde_json::to_string(&response) {
                            if let Some(redis_manager) = RedisManager::get_instance().try_lock() {
                                let _ = redis_manager.send_to_api(&msg.client_id, &json);
                            }
                        }
                    }
                },
            crate::types::MessageFromApi::ON_RAMP(_) => {
                if let crate::types::MessageFromApi::ON_RAMP(ramp_data) = &msg.message {
                    println!("Ramp data Amount: {}, user_id: {}, txn_id: {}", ramp_data.amount, ramp_data.user_id, ramp_data.txn_id);

                    let user_id = ramp_data.user_id.clone();
                    let amount: f64 = match ramp_data.amount.parse() {
                        Ok(val) => val,
                        Err(e) => {
                            println!("Failed to parse ramp_data.amount ('{}') as f64: {}", ramp_data.amount, e);
                            0.0
                        }
                    };

                    self.on_ramp(user_id, amount);
                }
                if let crate::types::MessageFromApi::GET_DEPTH(depth_data) = &msg.message {
                    println!("Market: {}", depth_data.market)
                }
            },
            crate::types::MessageFromApi::GET_OPEN_ORDERS(_) => {
                if let crate::types::MessageFromApi::GET_OPEN_ORDERS(open_order_data) = &msg.message {
                    let open_order_book = self.orderbooks.iter().find(|o| o.ticker() == open_order_data.market).expect("Orderbook not found");
                    let open_orders = open_order_book.getOpenOrders(open_order_data.user_id.clone());

                    let response = MessageToApi::OPEN_ORDERS(OpenOrdersPayload {
                        payload: open_orders,
                    });

                    if let Ok(json) = serde_json::to_string(&response) {
                        if let Some(redis_manager) = RedisManager::get_instance().try_lock() {
                            let _ = redis_manager.send_to_api(&msg.client_id, &json);
                        }
                    }
                }
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
    ) -> Result<(f64, Vec<Fill>, String), String> {
        let orderbook_index = match self.orderbooks
                            .iter()
                            .position(|o| o.ticker() == market) {
            Some(index) => index,
            None => return Err("No orderbook found".to_string())
        };

        if let Some((base, quote)) = market.split_once('-') {
            println!("Base: {}", base);   // ex: BTC
            println!("Quote: {}", quote); // ex: USD

            let btc = base.to_string();
            let usd = quote.to_string();

            println!("btc = {}, usd = {}", btc, usd);
        }

        let side_enum = match side {
            "buy" => Side::Buy,
            "sell" => Side::Sell,
            _ => return Err("Invalid side".to_string())
        };

        // do check and lock funds
        if let Some((base, quote)) = market.split_once('-') {
            self.check_and_lock_funds(base.to_string(), quote.to_string(), side_enum.clone(), user_id.to_string(), price.to_string(), quantity);
        }

        let new_order_id = Uuid::new_v4().to_string();
        let order = Order { 
            price: price, 
            quantity: quantity as f64, 
            order_id: new_order_id.clone(), 
            filled: 0.0, 
            side: side_enum.clone(), 
            user_id: user_id.to_string() 
        };
        let order_for_update = order.clone();

        // Extract base and quote from market string
        let (base, quote) = match market.split_once('-') {
            Some((b, q)) => (b.to_string(), q.to_string()),
            None => return Err("Invalid market format".to_string()),
        };

        let (executed_qty, fills) = self.orderbooks[orderbook_index].addOrder(order);
        self.update_balances(
            user_id.to_string(),
            base.clone(),
            quote.clone(),
            side_enum,
            executed_qty,
            fills.clone(),
        );
        // create db trades
        self.create_db_trades(fills.clone(), market, user_id.to_string());
        self.update_db_trades(
            order_for_update,
            executed_qty,
            fills.clone(),
            market.to_string(),
            user_id.to_string(),
        );
        self.publish_ws_depth_update(
            fills.clone(),
            price,
            side_enum.clone(),
            market.to_string(),
        );
        self.publish_ws_trades(
            fills.clone(),
            user_id.to_string(),
            market.to_string(),
        );
        println!(
            "Creating order: market={}, price={}, quantity={}, side={}, user_id={}",
            market, price, quantity, side, user_id
        );
        Ok((executed_qty, fills, new_order_id))
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

    pub fn update_balances(&mut self, user_id: String, base: String, quote: String, side: Side, executed_qty: f64, fills: Vec<Fill>) {
        if side == Side::Buy {
            fills.iter().for_each(|fill| {
                let price_f64: f64 = fill.price.parse().expect("Invalid price");
                let qty_f64: f64 = fill.qty as f64;

                let other_user_quote_balance = self.balances
                    .get_mut(&fill.other_user_id)
                    .expect("Other user not found")
                    .get_mut(&quote)
                    .expect("Quote asset not found");

                other_user_quote_balance.available += price_f64 * qty_f64;
                other_user_quote_balance.locked -= price_f64 * qty_f64;

                let user_base_balance = self.balances
                    .get_mut(&user_id)
                    .expect("User not found")
                    .get_mut(&base)
                    .expect("Base asset not found");

                user_base_balance.locked -= qty_f64;

                user_base_balance.available += qty_f64;
            });
    } else {
        fills.iter().for_each(|fill| {
            let price_f64: f64 = fill.price.parse().expect("Invalid price");
            let qty_f64: f64 = fill.qty as f64;

            let other_user_quote_balance = self.balances
                .get_mut(&fill.other_user_id)
                .expect("Other user not found")
                .get_mut(&quote)
                .expect("Quote asset not found");

            other_user_quote_balance.locked -= price_f64 * qty_f64;
            other_user_quote_balance.available += price_f64 * qty_f64;

            let user_base_balance = self.balances
                .get_mut(&user_id)
                .expect("User not found")
                .get_mut(&base)
                .expect("Base asset not found");

            user_base_balance.locked -= qty_f64;
            user_base_balance.available += qty_f64;
        });
        }
    }

    pub fn create_db_trades(&mut self, fills: Vec<Fill>, market: &str, user_id: String) {
        //TODO: implement
        fills.iter().for_each(|fills| {
            println!("Trade ID: {}", fills.trade_id);
            println!("Price: {}", fills.price);
            println!("Qty: {}", fills.qty);
            println!("Other User ID: {}", fills.other_user_id);
            println!("Market Order ID: {}", fills.market_order_id);

            //redis manager call type trade added
            let response = PushToDb::TRADE_ADDED(TRADEADDEDDATA {
                market: market.to_string(),
                id: fills.trade_id.to_string(),
                is_buyer_maker: fills.other_user_id == user_id,
                price: fills.price.to_string(),
                quantity: fills.qty.to_string(),
                quote_quantity: ((fills.qty as f64) * fills.price.parse::<f64>().unwrap_or(0.0)).to_string(),
                timestamp: chrono::Utc::now().timestamp(),
            });

            if let Ok(json) = serde_json::to_string(&response) {
                if let Some(redis_manager) = RedisManager::get_instance().try_lock() {
                    let _ = redis_manager.push_db(&json);
                }
            }
        })
    }

    pub fn update_db_trades(&mut self, order: Order, executed_qty: f64, fills: Vec<Fill>, market: String, user_id: String) {

        //redis manager call type ORDER_UPDATE here too
        let response = PushToDb::ORDER_UPDATE(ORDERUPDATEDATA {
            order_id: order.order_id.clone(),
            exec_qty: executed_qty,
            market: Some(market.to_string()),
            price: Some(order.price.to_string()),
            quantity: Some(order.quantity.to_string()),
            side: Some(order.side),
        });


        //TODO: implement
        fills.iter().for_each(|fill| {
            println!("Trade ID: {}", fill.trade_id);
            println!("Price: {}", fill.price);
            println!("Qty: {}", fill.qty);
            println!("Other User ID: {}", fill.other_user_id);
            println!("Market Order ID: {}", fill.market_order_id);
            println!("Order ID: {}", order.order_id);
            println!("Executed Qty: {}", executed_qty);
            println!("Market: {}", market);
            println!("User ID: {}", user_id);


            //redis manager call type ORDER_UPDATE
            let response = PushToDb::ORDER_UPDATE(ORDERUPDATEDATA {
                order_id: fill.market_order_id.clone(),
                exec_qty: fill.qty as f64,
                market: None,
                price: None,
                quantity: None,
                side: None,
            });


        })
    }

    pub fn publish_ws_trades(&mut self, fills: Vec<Fill>, user_id: String, market: String) {
        fills.iter().for_each(|fill| {
            let channel = format!("trade@{}", market);
            let trade_data = serde_json::json!({
                "stream": format!("trade@{}", market),
                "data": {
                    "e": "trade",
                    "t": fill.trade_id,
                    "m": fill.other_user_id == user_id,
                    "p": fill.price,
                    "q": fill.qty.to_string(),
                    "s": market,
                }
            });

            if let Ok(json) = serde_json::to_string(&trade_data) {
                if let Some(redis_manager) = RedisManager::get_instance().try_lock() {
                    let _ = redis_manager.publish_ws(&channel, &json);
                }
            }
        })
    }

    pub fn publish_ws_depth_update(&mut self, fills: Vec<Fill>, price: f64, side: Side, market: String) {
        println!("Price: {}", price);
        // println!("Side: {}", side);
        println!("Market: {}", market);

        let orderbook_index = match self.orderbooks
                            .iter()
                            .position(|o| o.ticker() == market) {
            Some(index) => index,
            None => return
        };

        let depth = self.orderbooks[orderbook_index].getDepth();
        if side == Side::Buy {
            let fill_prices: Vec<String> = fills.iter().map(|f| f.price.to_string()).collect();
            let updated_asks: Vec<&PriceLevel> = depth.asks.iter()
                .filter(|ask| fill_prices.contains(&ask.price))
                .collect();
            let updated_bids = depth.bids.iter().find(|bid| bid.price == price.to_string());

            // redis manager call publishMessage
            let channel = format!("depth@{}", market);
            let depth_data = serde_json::json!({
                "stream": format!("depth@{}", market),
                "data": {
                    "a": updated_asks,
                    "b": updated_bids.map_or_else(|| serde_json::Value::Array(vec![]), |bid| serde_json::json!([bid])),
                    "e": "depth",
                }
            });

            if let Ok(json) = serde_json::to_string(&depth_data) {
                if let Some(redis_manager) = RedisManager::get_instance().try_lock() {
                    let _ = redis_manager.publish_ws(&channel, &json);
                }
            }
        } else {
            let fill_prices: Vec<String> = fills.iter().map(|f| f.price.to_string()).collect();
            let updated_bids: Vec<&PriceLevel> = depth.bids.iter()
                .filter(|bid| fill_prices.contains(&bid.price))
                .collect();
            let updated_asks = depth.asks.iter().find(|ask| ask.price == price.to_string());

            // redis manager call publishMessage
            let channel = format!("depth@{}", market);
            let depth_data = serde_json::json!({
                "stream": format!("depth@{}", market),
                "data": {
                    "a": updated_asks.map_or_else(|| serde_json::Value::Array(vec![]), |ask| serde_json::json!([ask])),
                    "b": updated_bids,
                    "e": "depth",
                }
            });

            if let Ok(json) = serde_json::to_string(&depth_data) {
                if let Some(redis_manager) = RedisManager::get_instance().try_lock() {
                    let _ = redis_manager.publish_ws(&channel, &json);
                }
            }
        }
    }

    pub fn send_updated_depth(&mut self, price: String, market: String) {
        println!("Price: {}", price);
        println!("Market: {}", market);
        let orderbook = self.orderbooks
            .iter()
            .find(|o| o.ticker() == market)
            .expect("Orderbook not found");
        let depth = orderbook.getDepth();
        let updated_asks: Vec<&PriceLevel> = depth.asks.iter()
            .filter(|ask| ask.price == price)
            .collect();
        let updated_bids: Vec<&PriceLevel> = depth.bids.iter()
            .filter(|bid| bid.price == price)
            .collect();
            
        // redis manager call publishMessage
        let channel = format!("depth@{}", market);
        let depth_data = serde_json::json!({
            "stream": format!("depth@{}", market),
            "data": {
                "a": if updated_asks.len() > 0 {
                    serde_json::to_value(
                        updated_asks.iter().map(|ask| vec![ask.price.clone(), ask.quantity.clone()]).collect::<Vec<_>>()
                    ).unwrap()
                } else {
                    serde_json::json!([[price.clone(), "0"]])
                },
                "b": if updated_bids.len() > 0 {
                    serde_json::to_value(
                        updated_bids.iter().map(|bid| vec![bid.price.clone(), bid.quantity.clone()]).collect::<Vec<_>>()
                    ).unwrap()
                } else {
                    serde_json::json!([[price.clone(), "0"]])
                },
                "e": "depth",
            }
        });

        if let Ok(json) = serde_json::to_string(&depth_data) {
            if let Some(redis_manager) = RedisManager::get_instance().try_lock() {
                let _ = redis_manager.publish_ws(&channel, &json);
            }
        }

    }

    pub fn on_ramp(&mut self, user_id: String, amount: f64) {
        println!("User ID: {}", user_id);
        println!("Amount: {}", amount);

        let base_currency = "USD".to_string(); 
        match self.balances.get_mut(&user_id) {
            None => {
                let mut currency_map = std::collections::HashMap::new();
                currency_map.insert(
                    base_currency.clone(),
                    UserBalance {
                        available: amount,
                        locked: 0.0,
                    },
                );
                self.balances.insert(user_id.clone(), currency_map);
            }
            Some(user_balance) => {
                user_balance
                    .entry(base_currency.clone())
                    .and_modify(|bal| bal.available += amount)
                    .or_insert(UserBalance {
                        available: amount,
                        locked: 0.0,
                    });
            }
        }
    }


}