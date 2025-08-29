use std::{collections::HashMap, sync::Arc};

use anyhow::Ok;
use futures::StreamExt;
use redis::Client;
use tokio::task;

pub struct SubscriptionManager{
    subscription: HashMap<String, Vec<String>>,
    reverse_subscriptions: HashMap<String, Vec<String>>,
    redis_client: Client
}

impl SubscriptionManager {
    pub fn new(redis_url: &str) -> anyhow::Result<Self> {
        let client = Client::open(redis_url)?;
        Ok(Self {
            subscription: HashMap::new(),
            reverse_subscriptions: HashMap::new(),
            redis_client: client
        })
    }

    pub fn get_subscriptions(&self, user_id: &str) -> Vec<String> {
        self.subscription
            .get(user_id)
            .cloned()
            .unwrap_or_default()
    }

    pub async fn subscribe(&mut self, user_id: String, subscription: String) -> anyhow::Result<()> {
        // 1. Check if user already subscribed
        if let Some(user_subs) = self.subscription.get(&user_id){
            if user_subs.contains(&subscription) {
                return Ok(());
            }
        }

        // 2. Add to user -> topics map
        self.subscription
            .entry(user_id.clone())
            .or_default()
            .push(subscription.clone());

        // 3. Add to topic -> users map
        let user = self.reverse_subscriptions
            .entry(subscription.clone())
            .or_default();
        user.push(user_id.clone());

        // 4. If first user for this topic, subscribe in Redis
        if user.len() == 1 {
            let mut conn = self.redis_client.get_async_connection().await?;
            let mut pubsub = conn.into_pubsub();
            pubsub.subscribe(subscription.clone()).await?;

            // background task to handle messages from this channel
            task::spawn(async move {
                let mut pubsub = pubsub;
                while let Ok(msg) = pubsub.on_message().next().await {
                    let payload: String = msg.get_payload().unwrap_or_default();
                    let channel = msg.get_channel_name().to_string();
                    println!("Received from Redis: {payload} on {channel}");

                    let mut manager = manager.lock().await;
                    manager.send_message(&channel, &payload).await;
                }
            });
        }

        Ok(())
    }


    pub async fn unsubscribe(&mut self, user_id: String, subscription: String) -> anyhow::Result<()> {
        // 1. Remove from user -> topics map
        if let Some(user_subs) = self.subscription.get_mut(&user_id) {
            user_subs.retain(|s| s != &subscription);
        }

        // 2. Remove from topic -> users map
        if let Some(users) = self.reverse_subscriptions.get_mut(&subscription) {
            users.retain(|u| u != &user_id);

            // 3. If no users left for this topic, unsubscribe from Redis
            if users.is_empty() {
                let mut conn = self.redis_client.get_async_connection().await?;
                let mut pubsub = conn.into_pubsub();
                pubsub.unsubscribe(subscription.clone()).await?;
                println!("Unsubscribed from Redis channel: {}", subscription);
            }
        }

        Ok(())
    }

    pub async fn start_listener(&self) -> anyhow::Result<()> {
        let mut pubsub_conn = self.redis_client.get_async_connection().await
            .expect("Failed to get redis connection")
            .into_pubsub();
        let mut stream = pubsub_conn.on_message();
        let reverse_subs = Arc::clone(&self.reverse_subscriptions);
        let user_manager = Arc::clone(&self.user_manager);

        tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                let channel: String = msg.get_channel().unwrap();
                let payload: String = msg.get_payload().unwrap();

                // Parse the payload into JSON
                let parsed: serde_json::Value = match serde_json::from_str(&payload) {
                    Ok(val) => val,
                    Err(_) => continue,
                };

                // Find users susbcribed to this channel
                let users = {
                    let map = reverse_subs.lock().await;
                    map.get(&channel).cloned().unwrap_or_default()
                };

                for user_id in users {
                    if let Some(user) = UserManager::get_instance().get_user(&user_id).await {
                        user.emit(parsed.clone()).await;
                    }                
                }
            }
        });

        Ok(())
    }


}