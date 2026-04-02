use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Manages multiple channel integrations (Slack, Discord, etc.)
pub struct ChannelManager {
    channels: RwLock<HashMap<String, Arc<dyn Channel>>>,
}

impl ChannelManager {
    pub fn new() -> Self {
        Self {
            channels: RwLock::new(HashMap::new()),
        }
    }

    /// Register a new channel implementation
    pub async fn register_channel(&self, channel: Arc<dyn Channel>) {
        let mut map = self.channels.write().await;
        map.insert(channel.name().to_string(), channel);
    }

    /// Get a registered channel by name
    pub async fn get_channel(&self, name: &str) -> Option<Arc<dyn Channel>> {
        let map = self.channels.read().await;
        map.get(name).cloned()
    }

    /// Broadcast a message to all registered channels
    pub async fn broadcast(&self, message: SendMessage) -> Result<()> {
        let map = self.channels.read().await;
        for channel in map.values() {
            if let Err(e) = channel.send(message.clone()).await {
                tracing::error!(
                    channel = channel.name(),
                    error = %e,
                    "Failed to broadcast message"
                );
            }
        }
        Ok(())
    }
}
