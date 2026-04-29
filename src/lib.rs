// src/lib.rs

#![no_std] 

//! # 🛡️ Penta-V Kernel (Penta-V-Core)
//!
//! **The Sovereign Protocol for Geometric Stability & Thermal-Aware System Resilience.**
//!
//! Penta-V Kernel provides a mission-critical infrastructure for protecting system integrity
//! through **Geometric Load Balancing**. Instead of linear queuing, it treats system 
//! stressors as "Deficits" and dissipates them across N-dimensional Geometric Poles.

pub mod core;
pub mod shapes;
pub mod utils;
pub mod mesh; // Phase IV: Distributed Geometric Mesh Protocol

pub use core::{CORE_BASE, SECURE_CORE};
pub use shapes::GeometricBalancer;
pub use mesh::{StabilityPacket, MeshNode};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        // Critical: Ensure the core initializes at the defined base stability
        assert_eq!(CORE_BASE, 1.0);
        // Safety: Verify secure core threshold is non-zero
        assert!(SECURE_CORE > 0.0);
    }
}
