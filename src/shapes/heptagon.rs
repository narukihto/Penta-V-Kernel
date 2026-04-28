// src/shapes/heptagon.rs

//! # Heptagon Geometry
//! 
//! Implementation of the 7-pole geometric configuration.
//! Provides enhanced structural stability for systems transitioning 
//! from balanced to high-load states.

use super::GeometricBalancer;

/// Represents a Heptagon geometric configuration.
/// 
/// Poles: 7
/// Immunity Factor: 2.33
pub struct Heptagon;

impl GeometricBalancer for Heptagon {
    /// Returns the number of geometric poles for a Heptagon (7.0).
    #[inline(always)]
    fn poles(&self) -> f64 {
        7.0
    }

    /// Returns the canonical name of the shape.
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Heptagon"
    }
}
