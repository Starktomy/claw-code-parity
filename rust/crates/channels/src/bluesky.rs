use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct BlueskyChannel {
    // Configuration fields
}

impl BlueskyChannel {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for BlueskyChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Channel for BlueskyChannel {
    fn name(&self) -> &str {
        "bluesky"
    }

    async fn send(&self, message: SendMessage) -> Result<()> {
        tracing::info!(
            recipient = %message.recipient,
            content_length = message.content.len(),
            "Sending message to Bluesky"
        );
        Ok(())
    }
}
