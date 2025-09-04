use redis::RedisResult;
use crate::{engine::Engine, redis_manager::RedisManager};
use serde_json;

mod types;
mod engine;
mod redis_manager;

fn main() -> RedisResult<()> {
    let mut engine = Engine::new();
    println!("Engine initialized");

    let redis_manager = RedisManager::new();
    println!("Redis manager initialized");

    loop {
        if let Ok(Some(msg)) = redis_manager.pop_message() {
            println!("Received message: {}", msg);

            // Deserialize incoming order
            let order = serde_json::from_str(&msg).unwrap();

            // Process order inside engine
            engine.process(order);
        }
    }
}
