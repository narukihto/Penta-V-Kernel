// src/lib.rs

// Technical: Enable no_std only when NOT building for Python extensions
#![cfg_attr(not(feature = "extension-module"), no_std)]

//! # 🛡️ Penta-V Kernel (Penta-V-Core)
//!
//! **The Sovereign Protocol for Geometric Stability & Thermal-Aware System Resilience.**

pub mod core;
pub mod shapes;
pub mod utils;
pub mod mesh;

pub use crate::core::{CORE_BASE, SECURE_CORE};
pub use crate::shapes::GeometricBalancer;
pub use crate::mesh::{StabilityPacket, MeshNode};

// --- Python Bindings Section ---
// Only compiled when the "extension-module" feature is active (via Maturin/PyPI)
#[cfg(feature = "extension-module")]
use pyo3::prelude::*;

#[cfg(feature = "extension-module")]
#[pyfunction]
/// Python Bridge: Calculates geometric impact based on deficit and immunity factor
fn calculate_impact(deficit: f64, immunity: f64) -> PyResult<f64> {
    // Technical: Direct geometric dissipation logic bridge
    Ok((deficit * 0.02) / immunity)
}

#[cfg(feature = "extension-module")]
#[pymodule]
/// Python Module: Entry point for the penta_v_kernel PyPI package
fn penta_v_kernel(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_impact, m)?)?;
    // Exporting internal constants for Python accessibility
    m.add("SECURE_CORE", SECURE_CORE)?;
    Ok(())
}

// --- Panic Handler for no_std ---
// Technical: Required landing function for fatal errors in no_std builds.
// This block ensures the kernel halts safely without unwinding.
#[cfg(not(feature = "extension-module"))]
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &::core::panic::PanicInfo) -> ! {
    // Technical: Immediate halt loop to satisfy the abort requirement
    loop {}
}

// --- Internal Test Suite ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        // Critical: Ensure core initializes at defined base stability
        assert_eq!(CORE_BASE, 1.0);
        // Safety: Verify secure core threshold is operational
        assert!(SECURE_CORE > 0.0);
    }
}
