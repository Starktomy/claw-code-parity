use api::{MessageRequest, MessageResponse, ProviderClient, MessageStream};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;

/// A single route maps a hint/model to a specific provider and backend model.
#[derive(Debug, Clone)]
pub struct Route {
    pub provider_kind: String, // e.g., "anthropic", "xai", "openai"
    pub model: String,
}

/// Multi-model router for dispatching requests to the appropriate provider.
#[derive(Debug, Clone)]
pub struct Router {
    routes: HashMap<String, Route>,
    default_model: String,
}

impl Router {
    pub fn new(default_model: String) -> Self {
        Self {
            routes: HashMap::new(),
            default_model,
        }
    }

    pub fn add_route(&mut self, hint: String, route: Route) {
        self.routes.insert(hint, route);
    }

    pub fn resolve_route(&self, hint_or_model: &str) -> Route {
        if let Some(route) = self.routes.get(hint_or_model) {
            route.clone()
        } else {
            // Default fallback logic, in a real system we'd detect provider by model
            Route {
                provider_kind: "auto".to_string(),
                model: hint_or_model.to_string(),
            }
        }
    }

    pub fn get_client(&self, route: &Route) -> Result<ProviderClient> {
        // Simplified for scaffolding
        ProviderClient::from_model(&route.model).map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn send_message(&self, request: &MessageRequest) -> Result<MessageResponse> {
        let route = self.resolve_route(&request.model);
        let client = self.get_client(&route)?;
        client.send_message(request).await.map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn stream_message(&self, request: &MessageRequest) -> Result<MessageStream> {
        let route = self.resolve_route(&request.model);
        let client = self.get_client(&route)?;
        client.stream_message(request).await.map_err(|e| anyhow::anyhow!(e))
    }
}
