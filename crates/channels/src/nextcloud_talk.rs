use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct NextcloudTalkChannel {
    // Configuration fields
}

impl NextcloudTalkChannel {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for NextcloudTalkChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Channel for NextcloudTalkChannel {
    fn name(&self) -> &str {
        "nextcloud_talk"
    }

    async fn send(&self, message: SendMessage) -> Result<()> {
        tracing::info!(
            recipient = %message.recipient,
            content_length = message.content.len(),
            "Sending message to NextcloudTalk"
        );
        Ok(())
    }
}
