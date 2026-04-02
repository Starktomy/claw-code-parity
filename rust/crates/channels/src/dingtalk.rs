use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct DingtalkChannel {
    // Configuration fields
}

impl DingtalkChannel {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for DingtalkChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Channel for DingtalkChannel {
    fn name(&self) -> &str {
        "dingtalk"
    }

    async fn send(&self, message: SendMessage) -> Result<()> {
        tracing::info!(
            recipient = %message.recipient,
            content_length = message.content.len(),
            "Sending message to Dingtalk"
        );
        Ok(())
    }
}
