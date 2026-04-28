// src/shapes/octagon.rs

//! # Octagon Geometry
//! 
//! Implementation of the 8-pole geometric configuration.
//! This shape provides advanced resilience, serving as a critical buffer 
//! between standard operations and high-stress system states.

use super::GeometricBalancer;

/// Represents an Octagon geometric configuration.
/// 
/// Poles: 8
/// Immunity Factor: 2.66
pub struct Octagon;

impl GeometricBalancer for Octagon {
    /// Returns the number of geometric poles for an Octagon (8.0).
    #[inline(always)]
    fn poles(&self) -> f64 {
        8.0
    }

    /// Returns the canonical name of the shape.
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Octagon"
    }
}
