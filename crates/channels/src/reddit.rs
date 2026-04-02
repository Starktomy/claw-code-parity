use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct RedditChannel {
    // Configuration fields
}

impl RedditChannel {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for RedditChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Channel for RedditChannel {
    fn name(&self) -> &str {
        "reddit"
    }

    async fn send(&self, message: SendMessage) -> Result<()> {
        tracing::info!(
            recipient = %message.recipient,
            content_length = message.content.len(),
            "Sending message to Reddit"
        );
        Ok(())
    }
}
