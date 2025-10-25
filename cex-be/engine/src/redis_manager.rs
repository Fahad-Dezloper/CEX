use futures_util::lock::Mutex;
use log::info;
use once_cell::sync::Lazy;
use redis::{Client, RedisResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum DbMessage {
    TradeAdded(TradeAdded),
    OrderUpdate(OrderUpdate)
}

#[derive(Serialize, Deserialize)]
pub struct TradeAdded {
    pub id: String,
    pub is_buyer_maker: bool,
    pub price: String,
    pub quote_quantity: String,
    pub timestamp: i64,
    pub market: String
}

#[derive(Serialize, Deserialize)]
pub struct OrderUpdate {
    pub order_id: String,
    pub exec_qty: f64,
    market: Option<String>,
    price: Option<String>,
    quantity: Option<String>,
    side: Option<Side>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Buy,
    Sell
}

pub struct RedisManager {
    redis_client: Client,
    ws_client: Client,
    db_client: Client
}

static INSTANCE: Lazy<Mutex<RedisManager>> = Lazy::new(|| {
    info!("Creating new RedisManager instance");
    Mutex::new(RedisManager::new())
});

impl RedisManager {
    pub fn new() -> Self {
        //redis 1 pubsub
        println!("Initializing redis manager");
        let redis_client = Client::open("redis://localhost:6379".to_string()).expect("Fail to connect redis client");
        //redis 2 pubsub
        let ws_client = Client::open("redis://localhost:6380".to_string()).expect("Fail to connect ws client");
        //redis 3 queue/db
        let db_client = Client::open("redis://localhost:6381".to_string()).expect("Fail to connect db client");

        Self {
            redis_client,
            ws_client,
            db_client,
        }
    }

    pub fn get_instance() -> &'static Mutex<RedisManager> {
        info!("Getting Redis instance");
        &INSTANCE
    }

    pub fn pop_message(&self) -> RedisResult<Option<String>> {
        let mut conn = self.redis_client.get_connection()?;
        let response: Option<(String, String)> = redis::cmd("BRPOP")
            .arg("messages")
            .arg(1) // block for 1 second
            .query(&mut conn)?;

        Ok(response.map(|(_, msg)| msg))
    }

    /// Publish updates for WebSocket consumers
    pub fn publish_ws(&self, channel: &str, payload: &str) -> RedisResult<()> {
        let mut conn = self.ws_client.get_connection()?;
        redis::cmd("PUBLISH").arg(channel).arg(payload).execute(&mut conn);
        Ok(())
    }

    /// Push events into DB queue for persistence
    pub fn push_db(&self, payload: &str) -> RedisResult<()> {
        let mut conn = self.db_client.get_connection()?;
        redis::cmd("RPUSH").arg("db_events").arg(payload).execute(&mut conn);
        Ok(())
    }

    /// Send message back to API via Redis
    pub fn send_to_api(&self, client_id: &str, message: &str) -> RedisResult<()> {
        let mut conn = self.redis_client.get_connection()?;
        let channel = format!("api_response:{}", client_id);
        redis::cmd("PUBLISH").arg(channel).arg(message).execute(&mut conn);
        Ok(())
    }

    pub fn get_cached_markets(&self) -> RedisResult<Option<String>> {
        let mut conn = self.redis_client.get_connection()?;
        let val: Option<String> = redis::cmd("GET").arg("markets:list").query(&mut conn).ok();
        Ok(val)
    }
}