use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use tokio_tungstenite::WebSocketStream;
use tokio::net::TcpStream;

use crate::user::User;
use crate::subscription_manager::SubscriptionManager;

pub struct UserManager {
    users: Mutex<HashMap<String, User>>,
    subscription_manager: Arc<Mutex<SubscriptionManager>>,
}

impl UserManager {
    pub fn new(subscription_manager: Arc<Mutex<SubscriptionManager>>) -> Arc<Self> {
        Arc::new(Self {
            users: Mutex::new(HashMap::new()),
            subscription_manager,
        })
    }

    pub async fn add_user(
        self: &Arc<Self>, 
        ws: WebSocketStream<TcpStream>
    ) -> String {
        let id = Self::random_id();
        let user = User::new(id.clone(), ws, Arc::clone(&self.subscription_manager));
        self.users.lock().await.insert(id.clone(), user);
        id
    }

    pub async fn remove_user(&self, id: &str) {
        self.users.lock().await.remove(id);
        // Notify subscription manager that user left
        if let Ok(mut manager) = self.subscription_manager.try_lock() {
            if let Err(e) = manager.user_left(id).await {
                eprintln!("Failed to notify subscription manager: {}", e);
            }
        }
    }

    pub async fn get_user(&self, id: &str) -> Option<&mut User> {
        // This is a simplified approach - in practice you might want to use a different pattern
        // since we can't return a mutable reference from a mutex guard easily
        None
    }

    pub async fn emit_to_user(&self, user_id: &str, message: crate::types::OutgoingMessage) -> anyhow::Result<()> {
        if let Some(user) = self.users.lock().await.get_mut(user_id) {
            user.emit(message).await
        } else {
            Err(anyhow::anyhow!("User not found: {}", user_id))
        }
    }

    fn random_id() -> String {
        Uuid::new_v4().to_string()
    }
}
