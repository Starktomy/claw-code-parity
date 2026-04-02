use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct MatrixChannel {
    // Configuration fields
}

impl MatrixChannel {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MatrixChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Channel for MatrixChannel {
    fn name(&self) -> &str {
        "matrix"
    }

    async fn send(&self, message: SendMessage) -> Result<()> {
        tracing::info!(
            recipient = %message.recipient,
            content_length = message.content.len(),
            "Sending message to Matrix"
        );
        Ok(())
    }
}
