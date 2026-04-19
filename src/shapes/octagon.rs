// src/shapes/octagon.rs

//! Implementation of the Octagon shape.
//! Poles: 8
//! Immunity Factor: 8 / 3 ≈ 2.66 (Advanced Resilience Tier)

use super::GeometricBalancer;

pub struct Octagon;

impl GeometricBalancer for Octagon {
    /// Returns the number of poles for an octagon (8.0).
    fn poles(&self) -> f64 {
        8.0
    }

    /// Returns the name of the shape.
    fn name(&self) -> &str {
        "Octagon"
    }
}
