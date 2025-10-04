mod model;

use diesel::{r2d2::{self, ConnectionManager}, PgConnection, prelude::*};
pub use model::*;
use redis::Client;
use validator::Validate;
use std::env;
use log::{info, error};
use bigdecimal::FromPrimitive;


pub mod schema;

pub use schema::*;
use serde::{Deserialize, Serialize};

pub type DbPool  = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize)]
pub enum DbMessage {
    TradeAdded(TradeMessage),
    OrderUpdate(OrderMessage)
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TradeMessage {
    #[validate(length(min = 1))]
    pub id: String,
    pub is_buyer_maker: bool,
    pub price: String,
    pub quantity: String,
    pub quote_quantity: String,
    pub timestamp: i64,
    pub market: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct OrderMessage {
    #[validate(length(min = 1))]
    pub order_id: String,
    pub executed_qty: f64,
    pub market: Option<String>,
    pub price: Option<String>,
    pub quantity: Option<String>,
    pub side: Option<String>,
}

pub fn establish_connection() -> DbPool {
    match dotenvy::dotenv() {
        Ok(_) => {
            let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let manager = ConnectionManager::<PgConnection>::new(database_url);
            r2d2::Pool::builder().build(manager).expect("Failed to create pool")
        }
        Err(e) => {
            panic!("Error loading .env file: {}", e);
        }
    }
}

fn process_message(message: DbMessage, pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = pool.get()?;
    
    match message {
        DbMessage::TradeAdded(trade_msg) => {
            let trade = Trade {
                id: uuid::Uuid::new_v4(),
                is_buyer_maker: trade_msg.is_buyer_maker,
                price: trade_msg.price,
                quantity: trade_msg.quantity,
                quote_quantity: trade_msg.quote_quantity,
                timestamp: chrono::DateTime::from_timestamp(trade_msg.timestamp, 0)
                    .ok_or("Invalid timestamp")?
                    .naive_utc(),
                market: trade_msg.market,
            };
            
            diesel::insert_into(trades::table)
                .values(&trade)
                .execute(&mut conn)?;
                
            info!("Trade inserted successfully: {:?}", trade.id);
        }
        DbMessage::OrderUpdate(order_msg) => {
            let order = Order {
                id: uuid::Uuid::new_v4(),
                executed_qty: bigdecimal::BigDecimal::from_f64(order_msg.executed_qty)
                    .ok_or("Invalid executed_qty value")?,
                market: order_msg.market.unwrap_or_default(),
                price: order_msg.price.unwrap_or_default(),
                quantity: order_msg.quantity.unwrap_or_default(),
                side: order_msg.side.unwrap_or_default(),
                created_at: chrono::Utc::now().naive_utc(),
            };
            
            diesel::insert_into(orders::table)
                .values(&order)
                .execute(&mut conn)?;
                
            info!("Order inserted successfully: {:?}", order.id);
        }
    }
    
    Ok(())
}

pub async fn add_db(pool: DbPool) {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    let client = Client::open(redis_url.as_str())
        .expect("Failed to open Redis client");

    let mut conn = client.get_connection().expect("Failed to get Redis connection");

    println!("database started");

    loop {
        let result: Option<String> = redis::cmd("BRPOP")
            .arg("db_processor")
            .arg(1)
            .query(&mut conn)
            .unwrap_or(None);

        if let Some(message) = result {
            match serde_json::from_str::<DbMessage>(&message) {
                Ok(message) => {
                    info!("Received message: {:?}", message);
                    match process_message(message, &pool) {
                        Ok(_) => {
                            info!("Message processed successfully");
                        }
                        Err(e) => {
                            error!("Error processing message: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Error deserializing message: {}", e);
                }
            }   
        } else {
            info!("Waiting for messages...");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}

