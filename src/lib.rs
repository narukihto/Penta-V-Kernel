// src/lib.rs

#![no_std] 

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
// src/lib.rs

//! # Penta-V Kernel
//!
//! A high-performance Geometric Stability Protocol implementation in Rust.
//! This crate provides the core infrastructure for protecting system integrity
//! through N-dimensional geometric load balancing and defensive decay management.

pub mod core;
pub mod shapes;
pub mod utils;

pub use core::{CORE_BASE, SECURE_CORE};
pub use shapes::GeometricBalancer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        // Verify the core starts at the defined base stability
        assert_eq!(CORE_BASE, 1.0);
        assert!(SECURE_CORE > 0.0);
    }
}
