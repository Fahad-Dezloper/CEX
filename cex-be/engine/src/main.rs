use redis::RedisResult;
use crate::{engine::Engine, redis_manager::RedisManager};
use serde_json;
use log::{info, warn, error};

mod types;
mod engine;
mod redis_manager;
mod orderbook;

fn main() -> RedisResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    dotenvy::dotenv().ok();

    info!("Starting CEX Matching Engine...");

    let mut engine = Engine::new();
    info!("Engine initialized successfully");

    let redis_manager = RedisManager::new();
    info!("Redis manager initialized successfully");

    info!("Matching engine is ready and listening for orders...");

    loop {
        match redis_manager.pop_message() {
            Ok(Some(msg)) => {
                info!("Received message from API");
                
                match serde_json::from_str::<crate::types::ProcessInput>(&msg) {
                    Ok(order) => {
                        info!("Processing order for client: {}", order.client_id);
                        engine.process(order);
                        info!("Order processed successfully");
                    }
                    Err(e) => {
                        error!("Failed to deserialize message: {}", e);
                        error!("Raw message: {}", msg);
                    }
                }
            }
            Ok(None) => {
                continue;
            }
            Err(e) => {
                error!("Error receiving message from Redis: {}", e);
                continue;
            }
        }
    }
}
