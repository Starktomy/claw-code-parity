use api::{MessageRequest, MessageResponse, MessageStream};
use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;
use crate::router::Router;

#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_retries: usize,
    pub base_delay: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(500),
        }
    }
}

/// A client wrapper that provides automatic retries and failover.
#[derive(Debug, Clone)]
pub struct ReliableClient {
    router: Router,
    policy: RetryPolicy,
}

impl ReliableClient {
    pub fn new(router: Router, policy: RetryPolicy) -> Self {
        Self { router, policy }
    }

    pub async fn send_message(&self, request: &MessageRequest) -> Result<MessageResponse> {
        let mut attempts = 0;
        let mut delay = self.policy.base_delay;

        loop {
            match self.router.send_message(request).await {
                Ok(res) => return Ok(res),
                Err(e) if attempts < self.policy.max_retries => {
                    tracing::warn!(
                        error = %e,
                        attempt = attempts + 1,
                        "Request failed, retrying..."
                    );
                    sleep(delay).await;
                    attempts += 1;
                    delay *= 2; // Exponential backoff
                }
                Err(e) => return Err(e),
            }
        }
    }

    pub async fn stream_message(&self, request: &MessageRequest) -> Result<MessageStream> {
        let mut attempts = 0;
        let mut delay = self.policy.base_delay;

        loop {
            match self.router.stream_message(request).await {
                Ok(stream) => return Ok(stream),
                Err(e) if attempts < self.policy.max_retries => {
                    tracing::warn!(
                        error = %e,
                        attempt = attempts + 1,
                        "Stream request failed, retrying..."
                    );
                    sleep(delay).await;
                    attempts += 1;
                    delay *= 2;
                }
                Err(e) => return Err(e),
            }
        }
    }
}
