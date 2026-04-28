// src/shapes/nonagon.rs

//! # Nonagon Geometry (Triple Stability Milestone)
//! 
//! Implementation of the 9-pole geometric configuration.
//! This shape represents a major stability milestone, providing three times 
//! the base immunity factor for high-traffic surge protection.

use super::GeometricBalancer;

/// Represents a Nonagon geometric configuration.
/// 
/// Poles: 9
/// Immunity Factor: 3.0
pub struct Nonagon;

impl GeometricBalancer for Nonagon {
    /// Returns the number of geometric poles for a Nonagon (9.0).
    #[inline(always)]
    fn poles(&self) -> f64 {
        9.0
    }

    /// Returns the canonical name of the shape.
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Nonagon"
    }
}
