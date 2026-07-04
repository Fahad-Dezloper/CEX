use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use redis::Client;
use tokio::sync::Mutex;
use uuid::Uuid;
use std::sync::Arc;
use futures_util::StreamExt;

use crate::types::MessageToEngine;


pub struct RedisManager {
    publisher: Mutex<MultiplexedConnection>,
    client: Client,
}

impl RedisManager {
    pub async fn new(redis_url: &str) -> redis::RedisResult<Arc<Self>> {
        let client = Client::open(redis_url)?;
        let conn = client.get_multiplexed_tokio_connection().await?;

        Ok(Arc::new(Self { publisher: Mutex::new(conn), client }))
    }

    pub fn get_random_client_id(&self) -> String {
        Uuid::new_v4().to_string()
    }

    pub async fn send_and_await(&self, msg: MessageToEngine) -> redis::RedisResult<String> {
        let id = self.get_random_client_id();
        let (mut sink, mut stream) = self.client.get_async_pubsub().await?.split();
        sink.subscribe(&id).await?;

        {
            let mut publisher = self.publisher.lock().await;
            // Serialize msg to a String before pushing to Redis
            let serialized_msg = serde_json::to_string(&msg)
                .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "Serialization error", e.to_string())))?;
            publisher.lpush::<_, _, ()>("message", serialized_msg).await?;
        }

        if let Some(message) = stream.next().await {
            let payload: String = message.get_payload()?;
            return Ok(payload);
        }

        Err(redis::RedisError::from((
            redis::ErrorKind::IoError,
            "No message received"
        )))
    }

    pub async fn cache_markets(&self, markets_json: &str) -> redis::RedisResult<()> {
        let mut publisher = self.publisher.lock().await;
        publisher.set::<_, _, ()>("markets:list", markets_json).await?;
        Ok(())
    }

    pub async fn get_cached_markets(&self) -> redis::RedisResult<Option<String>> {
        let mut publisher = self.publisher.lock().await;
        let val: Option<String> = publisher.get("markets:list").await?;
        Ok(val)
    }
}

