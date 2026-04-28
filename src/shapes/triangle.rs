// src/shapes/triangle.rs

//! # Triangle Geometry (Foundation Tier)
//! 
//! Implementation of the 3-pole geometric configuration.
//! This shape represents the foundational baseline of the Penta-V protocol,
//! providing the minimum unit of structural stability.

use super::GeometricBalancer;

/// Represents a Triangle geometric configuration.
/// 
/// Poles: 3
/// Immunity Factor: 1.0
pub struct Triangle;

impl GeometricBalancer for Triangle {
    /// Returns the number of geometric poles for a Triangle (3.0).
    #[inline(always)]
    fn poles(&self) -> f64 {
        3.0
    }

    /// Returns the canonical name of the shape.
    #[inline(always)]
    fn name(&self) -> &'static str {
        "Triangle"
    }
}
