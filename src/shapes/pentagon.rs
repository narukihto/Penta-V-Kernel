// src/shapes/pentagon.rs

//! # Pentagon Geometry
//! 
//! Implementation of the 5-pole geometric configuration.
//! Provides intermediate structural stability, acting as the bridge 
//! between foundational and balanced system tiers.

use super::GeometricBalancer;

/// Represents a Pentagon geometric configuration.
/// 
/// Poles: 5
/// Immunity Factor: 1.66
pub struct Pentagon;

impl GeometricBalancer for Pentagon {
    /// Returns the number of geometric poles for a Pentagon (5.0).
    #[inline(always)]
    fn poles(&self) -> f64 {
        5.0
    }

    /// Returns the canonical name of the shape.
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Pentagon"
    }
}
