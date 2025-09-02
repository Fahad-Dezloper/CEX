use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")] 
pub enum MessageFromApi {
    CREATE_ORDER(CreateOrderData),
    CANCEL_ORDER(CancelOrderData),
    ON_RAMP(ONRAMPDATA),
    GET_DEPTH(GETDEPTHDATA),      
    GET_OPEN_ORDERS,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessInput {
    pub message: MessageFromApi,
    pub client_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderData {
    market: String,
    price: String,
    quantity: String,
    side: Side,
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CancelOrderData {
    order_id: String,
    market: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ONRAMPDATA {
    amount: String,
    user_id: String,
    txn_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GETDEPTHDATA {
    market: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GETOPENORDERS {
    user_id: String,
    market: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Buy,
    Sell,
}