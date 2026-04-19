// src/shapes/decagon.rs

//! Implementation of the Decagon shape.
//! Poles: 10
//! Immunity Factor: 10 / 3 ≈ 3.33 (High-Resilience Tier)

use super::GeometricBalancer;

pub struct Decagon;

impl GeometricBalancer for Decagon {
    /// Returns the number of poles for a decagon (10.0).
    fn poles(&self) -> f64 {
        10.0
    }

    /// Returns the name of the shape.
    fn name(&self) -> &str {
        "Decagon"
    }
}
