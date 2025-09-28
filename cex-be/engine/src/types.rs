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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Buy,
    Sell,
}