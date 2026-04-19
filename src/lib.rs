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
