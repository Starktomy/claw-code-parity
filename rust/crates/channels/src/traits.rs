use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

/// A message received from or sent to a channel
#[derive(Debug, Clone)]
pub struct ChannelMessage {
    pub id: String,
    pub sender: String,
    pub reply_target: String,
    pub content: String,
    pub channel: String,
    pub timestamp: u64,
    /// Platform thread identifier (e.g. Slack `ts`, Discord thread ID).
    pub thread_ts: Option<String>,
    /// Thread scope identifier for interruption/cancellation grouping.
    pub interruption_scope_id: Option<String>,
}

/// Message to send through a channel
#[derive(Debug, Clone)]
pub struct SendMessage {
    pub content: String,
    pub recipient: String,
    pub subject: Option<String>,
    /// Platform thread identifier for threaded replies
    pub thread_ts: Option<String>,
    /// Optional cancellation token for interruptible delivery
    pub cancellation_token: Option<CancellationToken>,
}

impl SendMessage {
    /// Create a new message with content and recipient
    pub fn new(content: impl Into<String>, recipient: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            recipient: recipient.into(),
            subject: None,
            thread_ts: None,
            cancellation_token: None,
        }
    }

    pub fn with_thread(mut self, thread_ts: impl Into<String>) -> Self {
        self.thread_ts = Some(thread_ts.into());
        self
    }
}

/// Core trait that all platform integrations must implement
#[async_trait]
pub trait Channel: Send + Sync + std::fmt::Debug {
    /// Get the unique name of this channel instance
    fn name(&self) -> &str;

    /// Send a message through this channel
    async fn send(&self, message: SendMessage) -> anyhow::Result<()>;
}
