// src/lib.rs

// --- CRITICAL CI FIX ---
// Force Standard Library during tests and Python builds to satisfy the Unwind Requirement
// Only enforce no_std in pure bare-metal release builds
#![cfg_attr(all(not(feature = "extension-module"), not(test), not(debug_assertions)), no_std)]

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
/// Python Bridge: Calculates geometric impact
fn calculate_impact(deficit: f64, immunity: f64) -> PyResult<f64> {
    Ok((deficit * 0.02) / immunity)
}

#[cfg(feature = "extension-module")]
#[pymodule]
fn penta_v_kernel(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_impact, m)?)?;
    m.add("SECURE_CORE", SECURE_CORE)?;
    Ok(())
}

// --- Panic Handler for no_std ---
// Only active in pure production no_std environments
#[cfg(all(not(feature = "extension-module"), not(test), not(debug_assertions)))]
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
        assert_eq!(CORE_BASE, 1.0);
        assert!(SECURE_CORE > 0.0);
    }
}
