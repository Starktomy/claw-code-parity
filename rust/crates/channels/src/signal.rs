use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct SignalChannel {
    // Configuration fields
}

impl SignalChannel {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SignalChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Channel for SignalChannel {
    fn name(&self) -> &str {
        "signal"
    }

    async fn send(&self, message: SendMessage) -> Result<()> {
        tracing::info!(
            recipient = %message.recipient,
            content_length = message.content.len(),
            "Sending message to Signal"
        );
        Ok(())
    }
}
