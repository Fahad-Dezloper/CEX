use std::{thread, time::Duration};

use redis::{RedisResult, Commands};
use crate::engine::Engine;
mod types;


mod engine;
mod redis_manager;

fn main() -> RedisResult<()> {
    let engine = Engine::new();
    println!("hi there");

    let client = redis::Client::open("redis://127.0.0.1:6379")?;
    let mut con = client.get_connection()?;

    let pong: String = redis::cmd("PING").query(&mut con)?;
    println!("connected to redis: {}", pong);

    loop {
        let response: Option<(String, String)> = con.brpop("messages", 0.0)?;
        if let Some((_key, msg)) = response {
            // println!("Got: {}", msg);
            engine.process(serde_json::from_str(&msg).unwrap());
            // serde_json::from_str(&msg).unwrap()
        }
    }
}