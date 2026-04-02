use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct MattermostChannel {
    // Configuration fields
}

impl MattermostChannel {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MattermostChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Channel for MattermostChannel {
    fn name(&self) -> &str {
        "mattermost"
    }

    async fn send(&self, message: SendMessage) -> Result<()> {
        tracing::info!(
            recipient = %message.recipient,
            content_length = message.content.len(),
            "Sending message to Mattermost"
        );
        Ok(())
    }
}
