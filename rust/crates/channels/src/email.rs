use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct EmailChannel {
    // Configuration fields
}

impl EmailChannel {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for EmailChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Channel for EmailChannel {
    fn name(&self) -> &str {
        "email"
    }

    async fn send(&self, message: SendMessage) -> Result<()> {
        tracing::info!(
            recipient = %message.recipient,
            content_length = message.content.len(),
            "Sending message to Email"
        );
        Ok(())
    }
}
