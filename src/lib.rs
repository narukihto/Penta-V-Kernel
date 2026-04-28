// src/lib.rs

#![no_std] 
#![deny(missing_docs)] 

//! # 🛡️ Penta-V Kernel (Penta-V-Core)
//!
//! **The Sovereign Protocol for Geometric Stability & Thermal-Aware System Resilience.**
//!
//! Penta-V Kernel provides a mission-critical infrastructure for protecting system integrity
//! through **Geometric Load Balancing**. Instead of linear queuing, it treats system 
//! stressors as "Deficits" and dissipates them across N-dimensional Geometric Poles.
//!
//! ## Quick Example
//! ```rust
//! use penta_v_kernel::shapes::{Circle, GeometricBalancer};
//! 
//! let shield = Circle;
//! let deficit = 100.0;
//! let impact = (deficit * 0.02) / shield.immunity_factor();
//! 
//! println!("Dissipated Impact: {}", impact);
//! ```

pub mod core;
pub mod shapes;
pub mod utils;

pub use core::{CORE_BASE, SECURE_CORE};
pub use shapes::GeometricBalancer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_constants() {
       
        assert_eq!(CORE_BASE, 1.0, "Core base must be normalized to 1.0");
        assert!(SECURE_CORE > 0.0, "Secure core threshold must be positive");
        assert!(SECURE_CORE < CORE_BASE, "Secure core must be below base stability");
    }
}
