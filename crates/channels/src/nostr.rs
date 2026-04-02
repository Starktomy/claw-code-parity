use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct NostrChannel {
    // Configuration fields
}

impl NostrChannel {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for NostrChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Channel for NostrChannel {
    fn name(&self) -> &str {
        "nostr"
    }

    async fn send(&self, message: SendMessage) -> Result<()> {
        tracing::info!(
            recipient = %message.recipient,
            content_length = message.content.len(),
            "Sending message to Nostr"
        );
        Ok(())
    }
}
