use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Represents an API Key record for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRecord {
    pub id: String,
    pub key_hash: String,
    pub name: String,
    pub created_at: u64,
}

/// Manages API keys for multi-tenant or multi-user setups
pub struct ApiKeyManager {
    keys: RwLock<HashMap<String, KeyRecord>>,
}

impl ApiKeyManager {
    pub fn new() -> Self {
        Self {
            keys: RwLock::new(HashMap::new()),
        }
    }

    pub async fn add_key(&self, record: KeyRecord) {
        let mut keys = self.keys.write().await;
        keys.insert(record.id.clone(), record);
    }

    pub async fn verify_key(&self, provided_key: &str) -> bool {
        // In a real implementation, this would hash the provided_key 
        // and check against the stored key_hash in the database.
        // For scaffold:
        let keys = self.keys.read().await;
        keys.values().any(|k| k.key_hash == provided_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_and_verify_key() {
        let manager = ApiKeyManager::new();
        let record = KeyRecord {
            id: "key-1".to_string(),
            key_hash: "hashed-secret-123".to_string(),
            name: "test-key".to_string(),
            created_at: 0,
        };

        manager.add_key(record).await;

        assert!(manager.verify_key("hashed-secret-123").await);
        assert!(!manager.verify_key("wrong-secret").await);
    }
}
