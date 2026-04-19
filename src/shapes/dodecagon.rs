// src/shapes/dodecagon.rs

//! Implementation of the Dodecagon shape.
//! Poles: 12
//! Immunity Factor: 12 / 3 = 4.0 (Quad-Stability Peak)

use super::GeometricBalancer;

pub struct Dodecagon;

impl GeometricBalancer for Dodecagon {
    /// Returns the number of poles for a dodecagon (12.0).
    fn poles(&self) -> f64 {
        12.0
    }

    /// Returns the name of the shape.
    fn name(&self) -> &str {
        "Dodecagon"
    }
}
