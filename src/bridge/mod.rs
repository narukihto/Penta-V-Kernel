// src/bridge/mod.rs

//! # Penta-Pack: Sovereign Bridge Module
//! 
//! This module orchestrates the packaging of Python logic into a standalone 
//! kernel binary, ensuring secure deployment and environmental independence.

pub mod packer;
pub mod security;

pub use packer::SovereignPacker;
pub use security::StructuralGuard;

/// Configuration for the sovereign deployment process.
pub struct BridgeConfig {
    /// If true, embeds the Python interpreter directly into the binary.
    pub self_contained: bool,
    /// Encryption level for the structural obfuscation (0.0 to 1.0).
    pub security_tier: f64,
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            self_contained: true,
            security_tier: 0.8, // High-security by default
        }
    }
}
