// src/shapes/decagon.rs

//! # Decagon Geometry
//! 
//! Provides a high-resilience structural dissipation tier using 10 poles.
//! This shape is deployed during extreme traffic surges to maintain stability.

use super::GeometricBalancer;

/// Represents a Decagon geometric configuration.
/// 
/// Poles: 10
/// Immunity Factor: 3.33
pub struct Decagon;

impl GeometricBalancer for Decagon {
    /// Returns the number of geometric poles for a Decagon (10.0).
    #[inline(always)]
    fn poles(&self) -> f64 {
        10.0
    }

    /// Returns the canonical name of the shape.
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Decagon"
    }
}
