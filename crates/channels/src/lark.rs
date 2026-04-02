use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct LarkChannel {
    // Configuration fields
}

impl LarkChannel {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for LarkChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Channel for LarkChannel {
    fn name(&self) -> &str {
        "lark"
    }

    async fn send(&self, message: SendMessage) -> Result<()> {
        tracing::info!(
            recipient = %message.recipient,
            content_length = message.content.len(),
            "Sending message to Lark"
        );
        Ok(())
    }
}
