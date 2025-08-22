use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Buy,
    Sell
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolData {
    pub market: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderData {
    pub market: String,
    pub price: f64,
    pub quantity: f64,
    pub side: Side,
    pub user_id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteOrderData {
    pub market: String,
    pub order_id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOpenOrder {
    pub user_id: String,
    pub market: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EngineData {
    Symbol(SymbolData),
    Order(CreateOrderData),
    DeleteOrder(DeleteOrderData),
    OpenOrder(GetOpenOrder)
}

#[derive(Serialize, Deserialize)]

pub struct Process {
    #[serde(rename = "type")]
    pub type_ : String,
    pub message: EngineData,
    pub client_id: String
}