use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct WecomChannel {
    // Configuration fields
}

impl WecomChannel {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for WecomChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Channel for WecomChannel {
    fn name(&self) -> &str {
        "wecom"
    }

    async fn send(&self, message: SendMessage) -> Result<()> {
        tracing::info!(
            recipient = %message.recipient,
            content_length = message.content.len(),
            "Sending message to Wecom"
        );
        Ok(())
    }
}
