use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct IrcChannel {
    // Configuration fields
}

impl IrcChannel {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for IrcChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Channel for IrcChannel {
    fn name(&self) -> &str {
        "irc"
    }

    async fn send(&self, message: SendMessage) -> Result<()> {
        tracing::info!(
            recipient = %message.recipient,
            content_length = message.content.len(),
            "Sending message to Irc"
        );
        Ok(())
    }
}
