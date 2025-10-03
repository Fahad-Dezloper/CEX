use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")] 
pub enum MessageFromApi {
    CREATE_ORDER(CreateOrderData),
    CANCEL_ORDER(CancelOrderData),
    ON_RAMP(ONRAMPDATA),
    GET_DEPTH(GETDEPTHDATA),      
    GET_OPEN_ORDERS(GETOPENORDERS),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum PushToDb {
    TRADE_ADDED(TRADEADDEDDATA),
    ORDER_UPDATE(ORDERUPDATEDATA),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessInput {
    pub message: MessageFromApi,
    pub client_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderData {
    pub market: String,
    pub price: String,
    pub quantity: String,
    pub side: Side,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelOrderData {
    pub order_id: String,
    pub market: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ONRAMPDATA {
    pub amount: String,
    pub user_id: String,
    pub txn_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GETDEPTHDATA {
    pub market: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GETOPENORDERS {
    pub user_id: String,
    pub market: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum MessageToApi {
    ORDER_PLACED(OrderPlacedPayload),
    ORDER_CANCELLED(OrderCancelledPayload),
    OPEN_ORDERS(OpenOrdersPayload),
    DEPTH(DepthPayload),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderPlacedPayload {
    pub order_id: String,
    pub executed_qty: f64,
    pub fills: Vec<FillResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderCancelledPayload {
    pub order_id: String,
    pub executed_qty: f64,
    pub remaining_qty: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FillResponse {
    pub price: String,
    pub qty: u64,
    pub trade_id: u64,
    pub other_user_id: String,
    pub market_order_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenOrdersPayload {
    pub payload: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DepthPayload {
    pub payload: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TRADEADDEDDATA {
    pub market: String,
    pub id: String,
    pub is_buyer_maker: bool,
    pub price: String,
    pub quantity: String,
    pub quote_quantity: String,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ORDERUPDATEDATA {
    pub order_id: String,
    pub exec_qty: f64,
    pub market: Option<String>,
    pub price: Option<String>,
    pub quantity: Option<String>,
    pub side: Option<Side>,
}