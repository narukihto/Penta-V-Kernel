// src/shapes/hexagon.rs

//! # Hexagon Geometry
//! 
//! Implementation of the 6-pole geometric configuration.
//! Known for high structural stability, the Hexagon is the standard 
//! operating tier for balanced system loads.

use super::GeometricBalancer;

/// Represents a Hexagon geometric configuration.
/// 
/// Poles: 6
/// Immunity Factor: 2.0
pub struct Hexagon;

impl GeometricBalancer for Hexagon {
    /// Returns the number of geometric poles for a Hexagon (6.0).
    #[inline(always)]
    fn poles(&self) -> f64 {
        6.0
    }

    /// Returns the canonical name of the shape.
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Hexagon"
    }
}
