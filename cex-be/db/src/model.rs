use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::{Associations, Identifiable, Insertable, Queryable};
use diesel::sql_types::Jsonb;
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::pg::Pg;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::{trades, orders, users, markets, user_assets};

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Identifiable)]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
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

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
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

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = markets)]
pub struct Market {
    pub id: Uuid,
    pub base_asset: String,
    pub quote_asset: String,
    pub symbol: String,
    pub enabled: bool,
    pub price_precision: i32,
    pub quantity_precision: i32,
    pub min_price: f64,
    pub max_price: f64,
    pub min_order_size: f64,
    pub max_order_size: f64,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Associations, Identifiable, Clone)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = user_assets)]
#[diesel(primary_key(id))]
pub struct UserAsset {
    pub id: Uuid,
    pub user_id: Uuid,
    pub symbol: String,
    pub amount: f64,
}

