use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct WhatsappChannel {
    // Configuration fields
}

impl WhatsappChannel {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for WhatsappChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Channel for WhatsappChannel {
    fn name(&self) -> &str {
        "whatsapp"
    }

    async fn send(&self, message: SendMessage) -> Result<()> {
        tracing::info!(
            recipient = %message.recipient,
            content_length = message.content.len(),
            "Sending message to Whatsapp"
        );
        Ok(())
    }
}
