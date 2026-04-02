use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct QqChannel {
    // Configuration fields
}

impl QqChannel {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for QqChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Channel for QqChannel {
    fn name(&self) -> &str {
        "qq"
    }

    async fn send(&self, message: SendMessage) -> Result<()> {
        tracing::info!(
            recipient = %message.recipient,
            content_length = message.content.len(),
            "Sending message to Qq"
        );
        Ok(())
    }
}
