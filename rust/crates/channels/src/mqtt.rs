use crate::traits::{Channel, SendMessage};
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct MqttChannel {
    // Configuration fields
}

impl MqttChannel {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MqttChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Channel for MqttChannel {
    fn name(&self) -> &str {
        "mqtt"
    }

    async fn send(&self, message: SendMessage) -> Result<()> {
        tracing::info!(
            recipient = %message.recipient,
            content_length = message.content.len(),
            "Sending message to Mqtt"
        );
        Ok(())
    }
}
