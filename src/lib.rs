// src/lib.rs

#![no_std] 

//! # 🛡️ Penta-V Kernel (Penta-V-Core)
//!
//! **The Sovereign Protocol for Geometric Stability & Thermal-Aware System Resilience.**

pub mod core;
pub mod shapes;
pub mod utils;
pub mod mesh; // Phase IV: Distributed Geometric Mesh Protocol
pub mod resonance; // Phase VI: Hyperdimensional Resonance Lattice

pub use core::{CORE_BASE, SECURE_CORE};
pub use shapes::GeometricBalancer;
pub use mesh::{StabilityPacket, MeshNode};
pub use resonance::HyperdimensionalStabilizer;

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
