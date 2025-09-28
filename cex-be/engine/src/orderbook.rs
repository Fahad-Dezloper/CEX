use crate::{engine::Order, types::Side};

pub struct OrderBook {
    base_asset: String,
    quote_asset: String,
    bids: Vec<Order>,
    asks: Vec<Order>,
    last_trade_id: u64,
    current_price: f64,
}

pub struct Fill {
    price: String,
    qty: u64,
    trade_id: u64,
    other_user_id: String,
    market_order_id: String
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

    pub fn addOrder(&mut self, mut order: Order) -> (f64, Vec<String>) {
        if order.side == crate::types::Side::Buy {
            let (executed_qty, fills) = (0.0, Vec::new()); // TODO: self.matchBid(order);
            order.filled = executed_qty;
            if executed_qty == order.quantity {
                return (executed_qty, fills);
            }
            self.bids.push(order);
            return (executed_qty, fills);
        } else {
            let (executed_qty, fills) = (0.0, Vec::new()); // TODO: self.matchAsk(order);
            order.filled = executed_qty;
            if executed_qty == order.quantity {
                return (executed_qty, fills);
            }

            self.asks.push(order);
            return (executed_qty, fills);
        }
    }

    pub fn matchBid(&self, order: Order) -> (f64, Vec<String>) {
        (0.0, Vec::new()) // TODO: implement matching logic
    }

    pub fn matchAsk(&self, order: Order) -> (f64, Vec<String>) {
        (0.0, Vec::new()) // TODO: implement matching logic
    }

    pub fn getDepth() -> String {
        format!("get depth")
    }

    pub fn getOpenOrders(user_id: String) -> String {
        format!("get open order")
    }

    pub fn cancelBid(order: Order) -> String {
        format!("cancel bid")
    }

    pub fn cancelAsk(order: Order) -> String {
        format!("cancel ask")
    }


}