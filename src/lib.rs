// src/lib.rs

// --- CRITICAL CI & TARGET CONFIGURATION ---
// We keep the standard library active for Python extensions and data processing.
// Bare-metal support is reserved for the Phase VII Hardware Abstraction Layer.

//! # 🛡️ Penta-V Kernel (Penta-V-Core)
//!
//! **The Sovereign Protocol for Geometric Stability & Thermal-Aware System Resilience.**
//! Integrated with Phase VI: Hyperdimensional Resonance Lattice for micro-coherence.

pub mod core;
pub mod shapes;
pub mod utils;
pub mod mesh;
pub mod processing; 
pub mod bridge;     
pub mod resonance; // Phase VI: Integrated Sub-Geometric Substrate

// Sovereign Exports
pub use crate::core::{CORE_BASE, SECURE_CORE};
pub use crate::shapes::GeometricBalancer;
pub use crate::mesh::{StabilityPacket, MeshNode};
pub use crate::processing::PentaCleaner;
pub use crate::resonance::HyperdimensionalStabilizer; 
pub use crate::bridge::{
    SovereignPacker, 
    LogicSignature, 
    GeometricValidator, 
    HeartbeatMonitor
};

// --- Python Bindings Section ---
#[cfg(feature = "extension-module")]
use pyo3::prelude::*;

#[cfg(feature = "extension-module")]
#[pyfunction]
/// Python Bridge: Calculates geometric impact based on the sovereign decay formula.
fn calculate_impact(deficit: f64, immunity: f64) -> PyResult<f64> {
    // Formula: (Deficit * 0.02) / Immunity
    Ok((deficit * 0.02) / immunity)
}

#[cfg(feature = "extension-module")]
#[pymodule]
fn penta_v_kernel(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Core Constants & Functions
    m.add_function(wrap_pyfunction!(calculate_impact, m)?)?;
    m.add("SECURE_CORE", SECURE_CORE)?;
    m.add("CORE_BASE", CORE_BASE)?;

    // Registering the Sovereign Bridge Classes
    m.add_class::<LogicSignature>()?;
    m.add_class::<HeartbeatMonitor>()?;
    m.add_class::<SovereignPacker>()?;

    // Submodule Registration: bridge
    let bridge_mod = PyModule::new_bound(m.py(), "bridge")?;
    bridge_mod.add_class::<LogicSignature>()?;
    bridge_mod.add_class::<HeartbeatMonitor>()?;
    bridge_mod.add_class::<SovereignPacker>()?;
    m.add_submodule(&bridge_mod)?;

    Ok(())
}

// --- Internal Test Suite ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        assert_eq!(CORE_BASE, 1.0);
        assert!(SECURE_CORE > 0.0);
    }
}
