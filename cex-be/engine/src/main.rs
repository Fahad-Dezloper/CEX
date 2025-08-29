use redis::Commands;
use std::thread::sleep;
use std::time::Duration;

use crate::main_engine::Enginee;
use crate::types::Process;
mod main_engine;
mod types;
mod orderbook;
mod redis;


fn main() {
    let engine: Enginee = Enginee::new();

    let client = redis::Client::open("redis://127.0.0.1/").expect("failed to create Redis client");
    let mut conn = client
        .get_connection()
        .expect("failed to get Redis connection");
    println!("Connected to Redis");

    loop {
        let response: redis::RedisResult<Option<String>> = conn.rpop("message", None);

        if let Ok(Some(resp)) = response {
            match serde_json::from_str::<Process>(&resp) {
                Ok(json) => engine.process(json),
                Err(err) => eprintln!("Json Parse err {}", err)
            }
        } else {
            sleep(Duration::from_millis(100));
        }
        
    }
}
