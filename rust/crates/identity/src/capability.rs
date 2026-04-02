use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use parking_lot::RwLock;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    /// Can read files from the workspace
    ReadFiles,
    /// Can write/modify files in the workspace
    WriteFiles,
    /// Can execute shell commands
    ExecuteCommands,
    /// Can fetch external web pages
    WebFetch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityMode {
    AllowAll,
    DenyAll,
    Restrict,
}

#[derive(Debug)]
pub struct CapabilityRegistry {
    mode: CapabilityMode,
    allowed: RwLock<HashSet<Capability>>,
}

impl CapabilityRegistry {
    pub fn new(mode: CapabilityMode) -> Self {
        Self {
            mode,
            allowed: RwLock::new(HashSet::new()),
        }
    }

    pub fn grant(&self, cap: Capability) {
        let mut allowed = self.allowed.write();
        allowed.insert(cap);
    }

    pub fn revoke(&self, cap: &Capability) {
        let mut allowed = self.allowed.write();
        allowed.remove(cap);
    }

    pub fn can_execute(&self, cap: &Capability) -> bool {
        match self.mode {
            CapabilityMode::AllowAll => true,
            CapabilityMode::DenyAll => false,
            CapabilityMode::Restrict => {
                let allowed = self.allowed.read();
                allowed.contains(cap)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allow_all_mode() {
        let registry = CapabilityRegistry::new(CapabilityMode::AllowAll);
        assert!(registry.can_execute(&Capability::ExecuteCommands));
    }

    #[test]
    fn test_deny_all_mode() {
        let registry = CapabilityRegistry::new(CapabilityMode::DenyAll);
        registry.grant(Capability::ReadFiles); // Should have no effect
        assert!(!registry.can_execute(&Capability::ReadFiles));
    }

    #[test]
    fn test_restrict_mode() {
        let registry = CapabilityRegistry::new(CapabilityMode::Restrict);
        assert!(!registry.can_execute(&Capability::WebFetch));
        
        registry.grant(Capability::WebFetch);
        assert!(registry.can_execute(&Capability::WebFetch));
        
        registry.revoke(&Capability::WebFetch);
        assert!(!registry.can_execute(&Capability::WebFetch));
    }
}
