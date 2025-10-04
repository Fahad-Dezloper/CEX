use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::Queryable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[diesel(table_name = trades)]
pub struct Trade {
    pub id: Uuid,
    pub is_buyer_maker: bool,
    pub price: String,
    pub quantity: String,
    pub quote_quantity: String,
    pub timestamp: NaiveDateTime,
    pub market: String
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: Uuid,
    pub executed_qty: BigDecimal,
    pub market: String,
    pub price: String,
    pub quantity: String,
    pub side: String,
    pub created_at: NaiveDateTime,
}

