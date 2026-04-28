// src/shapes/square.rs

//! # Square Geometry
//! 
//! Implementation of the 4-pole geometric configuration.
//! Represents the transition from foundational stability to balanced 
//! structural resilience.

use super::GeometricBalancer;

/// Represents a Square geometric configuration.
/// 
/// Poles: 4
/// Immunity Factor: 1.33
pub struct Square;

impl GeometricBalancer for Square {
    /// Returns the number of geometric poles for a Square (4.0).
    #[inline(always)]
    fn poles(&self) -> f64 {
        4.0
    }

    /// Returns the canonical name of the shape.
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Square"
    }
}
