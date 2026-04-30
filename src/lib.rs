// src/lib.rs

// Technical: Enable no_std ONLY when NOT building for Python AND NOT running tests
#![cfg_attr(all(not(feature = "extension-module"), not(test)), no_std)]

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
#[cfg(feature = "extension-module")]
use pyo3::prelude::*;

#[cfg(feature = "extension-module")]
#[pyfunction]
/// Python Bridge: Calculates geometric impact based on deficit and immunity factor
fn calculate_impact(deficit: f64, immunity: f64) -> PyResult<f64> {
    Ok((deficit * 0.02) / immunity)
}

#[cfg(feature = "extension-module")]
#[pymodule]
/// Python Module: Defines the penta_v_kernel entry point for PyPI
fn penta_v_kernel(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_impact, m)?)?;
    m.add("SECURE_CORE", SECURE_CORE)?;
    Ok(())
}

// --- Panic Handler for no_std ---
// Technical: Only active in pure no_std builds (Not active during Python builds or Tests)
#[cfg(all(not(feature = "extension-module"), not(test)))]
#[panic_handler]
fn panic(_info: &::core::panic::PanicInfo) -> ! {
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
