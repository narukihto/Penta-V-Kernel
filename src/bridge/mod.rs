//! # Penta-Pack: Sovereign Bridge Module
//! 
//! This module orchestrates the packaging of Python logic into a standalone 
//! kernel binary, while managing geometric logic validation and legacy telemetry.

pub mod packer;
pub mod security;
pub mod validator;
pub mod telemetry;

pub use packer::SovereignPacker;
pub use security::StructuralGuard;
pub use validator::{LogicSignature, GeometricValidator};
pub use telemetry::HeartbeatMonitor;

use pyo3::prelude::*;

/// Configuration for the sovereign deployment process.
#[derive(Clone)]
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

/// Registers the bridge modules into the Python environment via PyO3.
#[pymodule]
pub fn penta_v_kernel(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Logic Validation & Telemetry Classes
    m.add_class::<LogicSignature>()?;
    m.add_class::<HeartbeatMonitor>()?;
    
    // Deployment & Packaging Classes
    m.add_class::<SovereignPacker>()?;
    
    Ok(())
}
