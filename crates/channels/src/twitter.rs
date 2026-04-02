use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct TwitterChannel {
    // Configuration fields
}

impl TwitterChannel {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for TwitterChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Channel for TwitterChannel {
    fn name(&self) -> &str {
        "twitter"
    }

    async fn send(&self, message: SendMessage) -> Result<()> {
        tracing::info!(
            recipient = %message.recipient,
            content_length = message.content.len(),
            "Sending message to Twitter"
        );
        Ok(())
    }
}
