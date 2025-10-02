use crate::{engine::Order, types::Side};

pub struct OrderBook {
    pub base_asset: String,
    pub quote_asset: String,
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
    pub last_trade_id: u64,
    pub current_price: f64,
}

pub struct OrderBookSnapshot<'a> {
    pub base_asset: &'a String,
    pub quote_asset: &'a String,
    pub bids: &'a Vec<Order>,
    pub asks: &'a Vec<Order>,
    pub last_trade_id: u64,
    pub current_price: f64,
}
#[derive(Clone, Debug)]
pub struct PriceLevel {
    pub price: String,
    pub quantity: String,
}

pub struct Depth {
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
}

#[derive(Clone, Debug)]
pub struct Fill {
    pub price: String,
    pub qty: u64,
    pub trade_id: u64,
    pub other_user_id: String,
    pub market_order_id: String
}


impl OrderBook {
    fn new(
        base_asset: String,
        quote_asset: String,
        bids: Vec<Order>,
        asks: Vec<Order>,
        last_trade_id: Option<u64>,
        current_price: Option<f64>,
    ) -> Self {
        Self {
            base_asset,
            quote_asset,
            bids,
            asks,
            last_trade_id: last_trade_id.unwrap_or(0), 
            current_price: current_price.unwrap_or(0.0),
        }
    }

    pub fn ticker(&self) -> String {
        format!("{}-{}", self.base_asset, self.quote_asset)
    }

    pub fn get_snapshot(&self) -> OrderBookSnapshot<'_> {
        OrderBookSnapshot {
            base_asset: &self.base_asset,
            quote_asset: &self.quote_asset,
            bids: &self.bids,
            asks: &self.asks,
            last_trade_id: self.last_trade_id,
            current_price: self.current_price,
        }
    }

    pub fn addOrder(&mut self, mut order: Order) -> (f64, Vec<Fill>) {
        if order.side == crate::types::Side::Buy {
            let (executed_qty, fills) = self.matchBid(order.clone());
            order.filled = executed_qty;
            if executed_qty < order.quantity {
                self.bids.push(order);
            }
            return (executed_qty, fills);
        } else {
            let (executed_qty, fills) = self.matchAsk(order.clone());
            order.filled = executed_qty;
            if executed_qty < order.quantity {
                self.asks.push(order);
            }
            return (executed_qty, fills);
        }
    }

    pub fn matchBid(&mut self, order: Order) -> (f64, Vec<Fill>) {
        let mut fills: Vec<Fill> = Vec::new();
        let mut executed_qty: f64 = 0.0;
        
        // Match against asks (sell orders)
        let mut i = 0;
        while i < self.asks.len() && executed_qty < order.quantity {
            let ask = &mut self.asks[i];
            if ask.price <= order.price {
                let fill_qty = (ask.quantity - ask.filled).min(order.quantity - executed_qty);
                executed_qty += fill_qty;
                ask.filled += fill_qty;
                
                fills.push(Fill {
                    price: ask.price.to_string(),
                    qty: fill_qty as u64,
                    trade_id: self.last_trade_id + 1,
                    other_user_id: ask.user_id.clone(),
                    market_order_id: ask.order_id.clone(),
                });
                
                // Remove fully filled asks
                if ask.filled >= ask.quantity {
                    self.asks.remove(i);
                } else {
                    i += 1;
                }
            } else {
                break; // No more matching asks
            }
        }
        
        (executed_qty, fills)
    }

    pub fn matchAsk(&mut self, order: Order) -> (f64, Vec<Fill>) {
        let mut fills: Vec<Fill> = Vec::new();
        let mut executed_qty: f64 = 0.0;
        
        // Match against bids (buy orders)
        let mut i = 0;
        while i < self.bids.len() && executed_qty < order.quantity {
            let bid = &mut self.bids[i];
            if bid.price >= order.price {
                let fill_qty = (bid.quantity - bid.filled).min(order.quantity - executed_qty);
                executed_qty += fill_qty;
                bid.filled += fill_qty;
                
                fills.push(Fill {
                    price: bid.price.to_string(),
                    qty: fill_qty as u64,
                    trade_id: self.last_trade_id + 1,
                    other_user_id: bid.user_id.clone(),
                    market_order_id: bid.order_id.clone(),
                });
                
                // Remove fully filled bids
                if bid.filled >= bid.quantity {
                    self.bids.remove(i);
                } else {
                    i += 1;
                }
            } else {
                break; // No more matching bids
            }
        }
        
        (executed_qty, fills)
    }


    pub fn getDepth(&self) -> Depth {
        let mut bids: Vec<PriceLevel> = Vec::new();
        let mut asks: Vec<PriceLevel> = Vec::new();

        let mut bids_obj: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
        let mut asks_obj: std::collections::HashMap<String, f64> = std::collections::HashMap::new();

        // Aggregate bids by price
        for bid in self.bids.iter() {
            let price_key = bid.price.to_string();
            let remaining_qty = bid.quantity - bid.filled;
            if remaining_qty > 0.0 {
                *bids_obj.entry(price_key).or_insert(0.0) += remaining_qty;
            }
        }

        // Aggregate asks by price
        for ask in self.asks.iter() {
            let price_key = ask.price.to_string();
            let remaining_qty = ask.quantity - ask.filled;
            if remaining_qty > 0.0 {
                *asks_obj.entry(price_key).or_insert(0.0) += remaining_qty;
            }
        }

        // Convert to PriceLevel vectors
        for (price, quantity) in bids_obj {
            bids.push(PriceLevel {
                price: price.clone(),
                quantity: quantity.to_string(),
            });
        }

        for (price, quantity) in asks_obj {
            asks.push(PriceLevel {
                price: price.clone(),
                quantity: quantity.to_string(),
            });
        }

        // Sort bids (highest price first) and asks (lowest price first)
        bids.sort_by(|a, b| b.price.parse::<f64>().unwrap_or(0.0).partial_cmp(&a.price.parse::<f64>().unwrap_or(0.0)).unwrap());
        asks.sort_by(|a, b| a.price.parse::<f64>().unwrap_or(0.0).partial_cmp(&b.price.parse::<f64>().unwrap_or(0.0)).unwrap());

        Depth { bids, asks }
    }

    pub fn getOpenOrders(&self, user_id: String) -> String {
        let mut open_orders: Vec<&Order> = Vec::new();
        open_orders.extend(self.asks.iter().filter(|ask| ask.user_id == user_id));
        open_orders.extend(self.bids.iter().filter(|bid| bid.user_id == user_id));

        // TODO: Implement proper JSON serialization
        format!("Found {} open orders for user {}", open_orders.len(), user_id)
    }

    pub fn cancelBid(&mut self, order_id: &str) -> Option<f64> {
        if let Some(index) = self.bids.iter().position(|o| o.order_id == order_id) {
            let order = self.bids.remove(index);
            Some(order.price)
        } else {
            None
        }
    }

    pub fn cancelAsk(&mut self, order_id: &str) -> Option<f64> {
        if let Some(index) = self.asks.iter().position(|o| o.order_id == order_id) {
            let order = self.asks.remove(index);
            Some(order.price)
        } else {
            None
        }
    }


}