use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct ImessageChannel {
    // Configuration fields
}

impl ImessageChannel {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ImessageChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Channel for ImessageChannel {
    fn name(&self) -> &str {
        "imessage"
    }

    async fn send(&self, message: SendMessage) -> Result<()> {
        tracing::info!(
            recipient = %message.recipient,
            content_length = message.content.len(),
            "Sending message to Imessage"
        );
        Ok(())
    }
}
