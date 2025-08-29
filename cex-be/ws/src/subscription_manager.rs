use std::{collections::HashMap};

use anyhow::Ok;
use futures::{ StreamExt };
use redis::Client;
use tokio::task;
use std::sync::Arc;

use crate::user_manager::{UserManager};

pub struct SubscriptionManager{
    subscription: HashMap<String, Vec<String>>,
    reverse_subscriptions: HashMap<String, Vec<String>>,
    redis_client: Client,
    user_manager: Arc<UserManager>
}

impl SubscriptionManager {
    pub fn new(redis_url: &str, user_manager: Arc<UserManager>) -> anyhow::Result<Self> {
        let client = Client::open(redis_url)?;
        Ok(Self {
            subscription: HashMap::new(),
            reverse_subscriptions: HashMap::new(),
            redis_client: client,
            user_manager
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
            let user_manager = Arc::new(self.user_manager.clone());
            task::spawn(async move {
                let mut pubsub = pubsub;
                while let Some(msg) = pubsub.on_message().next().await {
                    let payload: String = msg.get_payload().unwrap_or_default();
                    let channel = msg.get_channel_name().to_string();
                    println!("Received from Redis: {payload} on {channel}");

                    let outgoing_msg = crate::types::OutgoingMessage {
                        event: channel.clone(),
                        data: payload.clone(),
                    };
                    
                    // Send to all users subscribed to this channel
                    if let Err(e) = user_manager.emit_to_user(&channel, outgoing_msg).await {
                        eprintln!("Failed to emit message: {}", e);
                    }
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

    pub async fn user_left(&mut self, user_id: &str) -> anyhow::Result<()> {
        if let Some(subs) = self.subscription.get(user_id) {
            // clone because we'll mutate while iterating
            let subs_to_remove = subs.clone();
            for s in subs_to_remove {
                self.unsubscribe(user_id.to_string(), s).await?;
            }
        }

        println!("user left {}", user_id);
        Ok(())
    }

    // redisCallbackHandler
    pub async fn handle_redis_message(&self, channel: &str, payload: &str) {
        if let Some(user_ids) = self.reverse_subscriptions.get(channel){
            for user_id in user_ids {
                let outgoing_msg = crate::types::OutgoingMessage {
                    event: channel.to_string(),
                    data: payload.to_string(),
                };
                if let Err(e) = self.user_manager.emit_to_user(user_id, outgoing_msg).await {
                    eprintln!("Failed to emit message to user {}: {}", user_id, e);
                }
            }
        }
    }

    pub async fn get_user(&self, user_id: &str) -> Option<&mut crate::user::User> {
        // This should return the actual user from your UserManager
        // For now, returning None as placeholder
        None
    }


}