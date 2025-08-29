use std::sync::Mutex;
use once_cell::sync::Lazy;
use log::info;
use redis::Client;

static INSTANCE: Lazy<Mutex<RedisManager>> = Lazy::new(|| {
    info!("Creating new RedisManager instance");
    Mutex::new(RedisManager::new())
});

pub struct RedisManager {
    redis_client: Client,
    ws_client: Client,
    db_client: Client
}

impl RedisManager {
    pub fn new() -> Self {
        info!("Initializing Redis Manager");
        // dotenv().ok();

        let redis_url = "redis://localhost:6379".to_string();
        let ws_url  = "redis://localhost:6380".to_string();
        let db_url  = "redis://localhost:6381".to_string();

        let redis_client = Client::open(redis_url.to_string())
                                            .expect("Failed to create Redis client");
        let ws_client = Client::open(ws_url.to_string())
                                            .expect("Failed to create Redis client");
        let db_client = Client::open(db_url.to_string())
                                            .expect("Failed to create Redis client");

        info!("Successfully created Redis clients");

        Self {
            redis_client,
            ws_client,
            db_client
        }
    }

    pub fn get_instance() -> &'static Mutex<RedisManager> {
        info!("Getting Redis instance");
        &INSTANCE
    }
}