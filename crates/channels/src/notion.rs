use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct NotionChannel {
    // Configuration fields
}

impl NotionChannel {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for NotionChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Channel for NotionChannel {
    fn name(&self) -> &str {
        "notion"
    }

    async fn send(&self, message: SendMessage) -> Result<()> {
        tracing::info!(
            recipient = %message.recipient,
            content_length = message.content.len(),
            "Sending message to Notion"
        );
        Ok(())
    }
}
