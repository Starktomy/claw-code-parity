use serde::{Deserialize, Serialize};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

/// WebAuthn relying party configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAuthnConfig {
    pub enabled: bool,
    pub rp_id: String,
    pub rp_origin: String,
    pub rp_name: String,
}

impl Default for WebAuthnConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            rp_id: "localhost".to_string(),
            rp_origin: "http://localhost:3000".to_string(),
            rp_name: "Claw Code Agent".to_string(),
        }
    }
}

pub struct WebAuthnManager {
    config: WebAuthnConfig,
}

impl WebAuthnManager {
    pub fn new(config: WebAuthnConfig) -> Self {
        Self { config }
    }

    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Generates a cryptographic challenge for authentication
    pub fn generate_challenge(&self) -> String {
        // Implementation stub. Would use `ring::rand::SecureRandom` 
        // to generate 32 bytes of entropy.
        let raw_bytes = b"dummy-challenge-32-bytes-stub-for-now";
        URL_SAFE_NO_PAD.encode(raw_bytes)
    }
}
