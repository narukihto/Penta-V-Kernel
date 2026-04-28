// src/shapes/dodecagon.rs

//! # Dodecagon Geometry (Quad-Stability Peak)
//! 
//! Implementation of the 12-pole geometric configuration.
//! This shape represents the zenith of finite stability before transitioning 
//! to asymptotic (Circle) defense modes.

use super::GeometricBalancer;

/// Represents a Dodecagon geometric configuration.
/// 
/// Poles: 12
/// Immunity Factor: 4.0
pub struct Dodecagon;

impl GeometricBalancer for Dodecagon {
    /// Returns the number of geometric poles for a Dodecagon (12.0).
    #[inline(always)]
    fn poles(&self) -> f64 {
        12.0
    }

    /// Returns the canonical name of the shape.
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Dodecagon"
    }
}
